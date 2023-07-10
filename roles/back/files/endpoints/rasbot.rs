use crate::endpoints::{gen_msg, BasicMessage};

use rocket::serde::json::Json;

use webhook::client::WebhookClient;
use lazy_static::lazy_static;
use std::{sync::{Arc, Mutex}, error::Error};

use urlencoding::decode;

lazy_static! {
    // TODO: make this use jackavery.ca as CDN for this image for longevity, though Discord tends to be fine
    static ref ICON_INFO: &'static str = "https://cdn.discordapp.com/attachments/488850419301220352/1127803740531990588/47289484.png";
    static ref ICON_ERR: &'static str = "https://cdn.discordapp.com/attachments/488850419301220352/1127803837948907530/47289484.png";
    static ref WEBHOOK_URL: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

pub async fn init(config: serde_yaml::Value) {
    // parse config
    // TODO: probably make this better. probably.
    let url: &str = config["endpoints"]["rasbot"]["webhook"].as_str().unwrap();
    let mut webhook_url = WEBHOOK_URL.lock().unwrap();
    *webhook_url = url.to_string();
}

async fn info(text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    webhook_send(*ICON_INFO, "info", text).await
}

async fn err(text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    webhook_send(*ICON_ERR, "err", text).await
}

async fn webhook_send(image: &str, mode: &str, text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    let webhook_url = WEBHOOK_URL.lock().unwrap().clone();
    let client: WebhookClient = WebhookClient::new(&webhook_url);
    let text_de = decode(text).expect("UTF-8");
    client.send(|message| message
        .username("rasbot logging")
        .avatar_url(*ICON_INFO)
        .embed(|embed| embed
            .description(&text_de)
            .author(format!("rasbot: {}", mode).as_str(), Some(String::from(image)), Some(String::from(image)))
    )).await?;

    Ok(())
}

#[get("/rasbot_notify/<mode>/<text>")]
pub async fn rasbot_notify(mode: &str, text: &str) -> Result<Json<BasicMessage>, Json<BasicMessage>> {
    match mode {
        "info" => {
            match info(text).await {
                Ok(..) => Ok(gen_msg(200, "ok".to_string())),
                Err(e) => Err(gen_msg(500, format!("failed: {}", e.to_string())))
            }
        }
        "err" => {
            match err(text).await {
                Ok(..) => Ok(gen_msg(200, "ok".to_string())),
                Err(e) => Err(gen_msg(500, format!("failed: {}", e.to_string())))
            }
        }
        _ => Err(gen_msg(500, format!("failed: bad mode")))
    }
}