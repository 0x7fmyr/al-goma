use std::{io::stdin, thread::sleep, time::Duration};


pub fn run_upload_setup() -> bool {
    let mut buffer = String::new();
    let mut answ = String::new();

    println!("\nThis will guide you through connecting al-goma to Google Tasks.\n");
    sleep(Duration::from_secs(5));
    println!("You will need a Google account.");
    sleep(Duration::from_secs(1));
    println!("Press enter to continue...");
    stdin().read_line(&mut buffer).ok();

    println!("\nStep 1: Go to console.cloud.google.com");
    sleep(Duration::from_secs(3));
    println!("- Create a new project (name it anything)");
    sleep(Duration::from_secs(1));
    println!("- In the left menu: APIs & Services > Library");
    sleep(Duration::from_secs(1));
    println!("- Search for \"Google Tasks API\" and enable it\n");
    sleep(Duration::from_secs(1));
    println!("Press enter to continue...");
    stdin().read_line(&mut buffer).ok();

    println!("\nStep 2: Create credentials");
    sleep(Duration::from_secs(3));
    println!("- Go to APIs & Services > Credentials");
    sleep(Duration::from_secs(1));
    println!("- Click \"Create Credentials\" > \"OAuth client ID\"");
    sleep(Duration::from_secs(1));
    println!("- If prompted, configure the consent screen first (External, fill in app name only)");
    sleep(Duration::from_secs(1));
    println!("- Application type: Desktop app");
    sleep(Duration::from_secs(1));
    println!("- Name it anything, click Create");
    sleep(Duration::from_secs(1));
    println!("- Click \"Download JSON\"\n");
    sleep(Duration::from_secs(1));
    println!("Press enter to continue...");
    sleep(Duration::from_secs(1));
    stdin().read_line(&mut buffer).ok();

    println!("\nStep 3: Place the file");
    sleep(Duration::from_secs(3));
    println!("- Rename the downloaded file to: clientsecret.json");
    sleep(Duration::from_secs(1));
    println!("- Move it to: ~/.config/al-goma/oauth/clientsecret.json\n");
    sleep(Duration::from_secs(1));
    println!("Press enter to continue...");
    stdin().read_line(&mut buffer).ok();

    println!("\nDone!");
    sleep(Duration::from_secs(2));
    println!("\nEnter 'y' to login or 'n' to exit ");

    stdin().read_line(&mut answ).ok();

    answ.trim() == "y"
}
