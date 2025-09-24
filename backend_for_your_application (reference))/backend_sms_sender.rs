use reqwest::Client;
use serde::Serialize;

#[derive(Serialize)]
struct SmsRequest<'a> {
    to: &'a str,
    message: &'a str,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sms_api_url = "https://<your_ngrok_url>.ngrok-free.app/v1/sms"; // to replace
    let auth_token = "your_token";

    let sms = SmsRequest {
        to: "+919123456789", // to replace
        message: "Your OTP is: 483920", // to replace
    };

    let client = Client::new();
    let res = client
        .post(sms_api_url)
        .bearer_auth(auth_token)
        .json(&sms)
        .send()
        .await?;

    if res.status().is_success() {
        println!("OTP sent successfully!");
    } else {
        eprintln!("Failed to send OTP: {:?}", res.text().await?);
    }

    Ok(())
}
