use std::{
    fmt::format,
    io::{self, Write},
    time::Duration,
};

use super::client::{Client, ResponseData};
use qrcode::{render::unicode, QrCode};
use reqwest::header::{self, USER_AGENT};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct Credential(Client);

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LoginStatus {
    pub url: Option<String>,
    pub code: i32,
    message: String,
    timestamp: i64,
    refersh_token: Option<String>,
}

impl Credential {
    pub fn new() -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            "Referer",
            header::HeaderValue::from_static("https://www.bilibili.com/"),
        );
        Self(Client::new(headers))
    }

    pub async fn get_web_qrcode(&self) -> Result<(), Box<dyn std::error::Error>> {
        let qr_code: Value = self
            .0
            .client
            .get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
            .send()
            .await?
            .json()
            .await?;
        let (url, qrcode_key) = (
            qr_code["data"]["url"].as_str().unwrap(),
            qr_code["data"]["qrcode_key"].as_str().unwrap(),
        );
        let image = QrCode::new(url)
            .expect("QrCode don't generat, please check generator url")
            .render::<unicode::Dense1x2>()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{image}");
        // let mut input = String::new();
        // loop {
        //     print!("Enter 'done' when you have scanned the QR code: ");
        //     io::stdout().flush().expect("Error flushing stdout");
        //     io::stdin()
        //         .read_line(&mut input)
        //         .expect("Error reading input");
        //     if input.trim() == "done" {
        //         break;
        //     }
        //     input.clear();
        // }
        tokio::time::sleep(Duration::from_secs(10)).await;
        let ResponseData{data:login_status,..}: ResponseData<LoginStatus> = self
         .0
         .client
         .get(format!("https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key={qrcode_key}"))
         .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36")
         .send()
         .await?
         .json()
         .await?;

        let value = match login_status.unwrap() {
            LoginStatus { code: 0, url, .. } => {
                println!("扫码成功，您已登录");
                url.unwrap()
            }
            LoginStatus { code, .. } => code.to_string(),
        };

        println!("{value}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test() -> Result<(), Box<dyn std::error::Error>> {
        let res = Credential::new();
        let s = res.get_web_qrcode().await?;
        println!("{s:?}");
        Ok(())
    }
}
