use base64::Engine;
use qrcode::{render::unicode, QrCode};
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let qrcode: serde_json::Value =
        reqwest::get("https://passport.bilibili.com/x/passport-login/web/qrcode/generate")
            .await?
            .json()
            .await?;
    let code = QrCode::new(qrcode["data"]["url"].as_str().unwrap()).unwrap();
    println!("{}", qrcode["data"]["qrcode_key"]);
    let image = code
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

    // thread::sleep(Duration::from_secs(1));

    let url = format!(
        "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key={}",
        qrcode["data"]["qrcode_key"]
    );

    let cookie: serde_json::Value = reqwest::Client::new().get(url).send().await?.json().await?;
    println!("cookie:{cookie}");
    Ok(())
}
