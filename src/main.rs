use std::io::{self, Write};

use base64::Engine;
use qrcode::{render::unicode, QrCode};
use reqwest::header::USER_AGENT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    test1().await;
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("输入");
    test2().await;
    Ok(())
}

async fn test1() {
    println!("fw1");
}

async fn test2() {
    println!("fw2");
}
