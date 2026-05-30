use serde::Deserialize;
use serde::Serialize;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use yup_oauth2::{InstalledFlowAuthenticator, authenticator_delegate::InstalledFlowDelegate};

use std::{fs, pin::Pin};

use crate::items::Ingredient;

use crate::app::{App, AppState};

pub struct AlgomaFlowDelegate {
    url_sender: Sender<String>,
    code_receiver: tokio::sync::Mutex<Receiver<String>>,
}

#[derive(Serialize)]
pub struct GoogleTasksListName {
    title: String,
}

#[derive(Deserialize)]
pub struct GoogleTaskListResponse {
    id: String,
}

#[derive(Serialize)]
pub struct GoogleTasksListItem {
    title: String,
}
#[derive(Debug, Clone, Copy)]
pub struct UploadProgress {
    pub procent: f64,
    pub done: bool,
}

impl App {
    pub fn upload_first_login(&mut self) {
        let (url_sender, url_receiver) = mpsc::channel(100);
        let (code_sender, code_receiver) = mpsc::channel(100);
        let (login_result_sender, login_result_receiver) = mpsc::channel(100);

        std::thread::spawn(move || {
            let result = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(login(url_sender, code_receiver));

            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(login_result_sender.send(result))
                .ok();
        });

        self.url_receiver = Some(url_receiver);
        self.code_sender = Some(code_sender);
        self.login_result_receiver = Some(login_result_receiver);

        self.state = AppState::UploadWaitingLoginUrl;
    }

    pub fn send_code(&mut self) {
        if let Some(sender) = &self.code_sender {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(sender.send(self.input.clone()))
                .ok();
        }
        self.input.clear();
        self.state = AppState::UploadLogginginWait
    }

    pub fn init_upload_list(&mut self) {
        self.state = AppState::Uploading;
        let shopping_list = self.shopping_list.clone();
        let input = self.input.clone();
        let (progress_sender, progress_receiver) = mpsc::channel(100);

        std::thread::spawn(move || {
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(upload(shopping_list, input, progress_sender))
                .ok();
        });

        self.progress_checker_receiver = Some(progress_receiver);
    }
}

pub fn does_token_exist() -> Result<bool, String> {
    let data_folder = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/");

    match data_folder.try_exists() {
        Ok(true) => match data_folder.join("tokencache.json").try_exists() {
            Ok(true) => return Ok(true),
            Ok(false) => return Ok(false),
            Err(_) => return Err("tokencache.json is corrupt!".to_string()),
        },
        Ok(false) => fs::create_dir_all(data_folder).expect("failed to make config dir..."),
        Err(_) => return Err("failed to find data path...".to_string()),
    };

    Ok(false)
}

impl InstalledFlowDelegate for AlgomaFlowDelegate {
    fn present_user_url<'a>(
        &'a self,
        url: &'a str,
        needs_code: bool,
    ) -> Pin<Box<dyn Future<Output = Result<String, String>> + Send + 'a>> {
        Box::pin(async move {
            match self.url_sender.send(url.to_string()).await {
                Ok(_) => {
                    if needs_code {
                        let mut receiver = self.code_receiver.lock().await;
                        let code = receiver.recv().await;
                        if code.is_none() {
                            return Err("no code received".to_string());
                        } else {
                            return Ok(code.unwrap());
                        }
                    } else {
                        return Ok(String::new());
                    }
                }
                Err(e) => return Err(format!("{}", e)),
            };
        })
    }
}

pub async fn login(
    url_sender: mpsc::Sender<String>,
    code_receiver: mpsc::Receiver<String>,
) -> Result<(), String> {
    let secret_path = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/clientsecret.json");

    let token_path = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/tokencache.json");

    let secret = match yup_oauth2::read_application_secret(secret_path).await {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };

    let login = match InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::Interactive,
    )
    .persist_tokens_to_disk(token_path)
    .flow_delegate(Box::new(AlgomaFlowDelegate {
        url_sender: url_sender,
        code_receiver: tokio::sync::Mutex::new(code_receiver),
    }))
    .build()
    .await
    {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };

    let scopes = &["https://www.googleapis.com/auth/tasks"];

    match login.token(scopes).await {
        Ok(_) => return Ok(()),
        Err(e) => return Err(format!("{}", e)),
    };
}

pub async fn upload(
    shopping_list: Vec<Ingredient>,
    input: String,
    progress_sender: Sender<UploadProgress>,
) -> Result<(), String> {
    let secret_path = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/clientsecret.json");

    let token_path = dirs::data_dir()
        .expect("failed to find data path...")
        .join("al-goma/tokencache.json");

    let secret = match yup_oauth2::read_application_secret(secret_path).await {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };

    let login = match InstalledFlowAuthenticator::builder(
        secret,
        yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
    )
    .persist_tokens_to_disk(token_path)
    .build()
    .await
    {
        Ok(s) => s,
        Err(e) => return Err(format!("{}", e)),
    };

    let scopes = &["https://www.googleapis.com/auth/tasks"];
    let token_string = match login.token(scopes).await {
        Ok(s) => s.token().unwrap().to_string(),
        Err(e) => return Err(format!("{}", e)),
    };

    let list_title = GoogleTasksListName { title: input };

    let client = reqwest::Client::new();

    let response = match client
        .post("https://tasks.googleapis.com/tasks/v1/users/@me/lists")
        .bearer_auth(&token_string)
        .json(&list_title)
        .send()
        .await
    {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    let list_response = match response.json::<GoogleTaskListResponse>().await {
        Ok(s) => s,
        Err(e) => return Err(e.to_string()),
    };

    let list_url = format!(
        "https://tasks.googleapis.com/tasks/v1/lists/{}/tasks",
        list_response.id
    );

    let mut progress = UploadProgress {
        procent: 0.0,
        done: false,
    };

    for (i, ingredient) in shopping_list.iter().rev().enumerate() {
        let item = GoogleTasksListItem {
            title: ingredient.name.clone(),
        };

        match client
            .post(&list_url)
            .bearer_auth(&token_string)
            .json(&item)
            .send()
            .await
        {
            Ok(_) => {
                progress.procent = i as f64 / shopping_list.len() as f64;

                progress_sender.send(progress.clone()).await.ok();

                continue;
            }
            Err(e) => return Err(e.to_string()),
        };
    }
    progress_sender
        .send(UploadProgress {
            procent: 1.0,
            done: true,
        })
        .await
        .ok();

    return Ok(());
}
