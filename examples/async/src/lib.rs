use spin_sdk::{wasip3, http::{EmptyBody, body::IncomingBodyExt, Request, send}};

wasip3::cli::command::export!(Main);

struct Main;

impl wasip3::exports::cli::run::Guest for Main {
    async fn run() -> Result<(), ()> {
        match main().await {
            Ok(()) => Ok(()),
            Err(e) => {
                eprintln!("{e}");
                Err(())
            }
        }
    }
}

async fn main() -> anyhow::Result<()> {
    let request = Request::get("https://myip.fermyon.app").body(EmptyBody::new())?;
    let response = send(request).await?;
    let response_bytes = response.into_body().bytes().await?;
    let response_text = String::from_utf8_lossy(&response_bytes.as_ref());
    println!("Your IP is: {}", response_text);
    Ok(())
}
