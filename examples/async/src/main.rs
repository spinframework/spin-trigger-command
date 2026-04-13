use spin_sdk::http::{EmptyBody, body::IncomingBodyExt, Request, send};

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let request = Request::get("https://myip.fermyon.app").body(EmptyBody::new())?;
    let response = send(request).await?;
    let response_bytes = response.into_body().bytes().await?;
    let response_text = String::from_utf8_lossy(&response_bytes.as_ref());
    println!("Your IP is: {}", response_text);
    Ok(())
}
