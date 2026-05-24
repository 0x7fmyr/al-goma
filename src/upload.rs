use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

use yup_oauth2::{
    ApplicationSecret, InstalledFlowAuthenticator, authenticator_delegate::InstalledFlowDelegate,
};

use std::{fs, pin::Pin};

pub struct AlgomaFlowDelegate {
    url_sender: Sender<String>,
    code_receiver: tokio::sync::Mutex<Receiver<String>>,
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

//https://docs.rs/yup-oauth2/12.1.2/yup_oauth2/

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
