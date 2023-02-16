use reqwest::header;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug)]
pub struct Client {
    pub client: reqwest::Client,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResponseData<T = Value> {
    pub code: i32,
    pub data: Option<T>,
    message: String,
    ttl: Option<u8>,
}

impl Client {
    pub fn new(mut headers: header::HeaderMap) -> Self {
        headers.insert("Connection", header::HeaderValue::from_static("keep-alive"));

        Client { 
            client: reqwest::Client::builder()
              .user_agent(
                "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36"
               )
              .default_headers(headers)
              .build()
              .unwrap()
        }
    }
}

