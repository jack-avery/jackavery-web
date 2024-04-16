use crate::endpoints::BasicMessage;

use rocket::serde::json::Json;

use lazy_static::lazy_static;
use serde::Deserialize;
use std::{
    error::Error,
    sync::{Arc, Mutex},
};
use webhook::client::WebhookClient;

use urlencoding::decode;

lazy_static! {
    // TODO: make this use jackavery.ca as CDN for this image for longevity, though Discord tends to be fine
    static ref ICON_INFO: &'static str = "https://cdn.discordapp.com/attachments/488850419301220352/1127803740531990588/47289484.png";
    static ref ICON_ERR: &'static str = "https://cdn.discordapp.com/attachments/488850419301220352/1127803837948907530/47289484.png";
    static ref WEBHOOK_URL: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

#[derive(Debug, Clone, Deserialize)]
pub struct Notify {
    mode: String,
    message: String
}

pub async fn init(config: &serde_yaml::Value) {
    let url: &str = config["webhook"].as_str().unwrap();
    let mut webhook_url = WEBHOOK_URL.lock().unwrap();
    *webhook_url = url.to_string();
}

async fn info(text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    webhook_send(*ICON_INFO, "info", text).await
}

async fn err(text: &str) -> Result<(), Box<dyn Error + Send + Sync>> {
    webhook_send(*ICON_ERR, "err", text).await
}

async fn webhook_send(
    image: &str,
    mode: &str,
    text: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let webhook_url: String = WEBHOOK_URL.lock().unwrap().clone();
    let client: WebhookClient = WebhookClient::new(&webhook_url);
    let text_de: std::borrow::Cow<'_, str> = decode(text).expect("UTF-8");
    dbg!(&webhook_url, &text_de);
    client
        .send(|message| {
            message
                .username("rasbot logging")
                .avatar_url(*ICON_INFO)
                .embed(|embed| {
                    embed.description(&text_de).author(
                        format!("rasbot: {}", mode).as_str(),
                        Some(String::from(image)),
                        Some(String::from(image)),
                    )
                })
        })
        .await?;

    Ok(())
}

#[post("/rasbot_notify", format = "json", data = "<notif>")]
pub async fn rasbot_notify(notif: Json<Notify>) -> Result<Json<BasicMessage>, Json<BasicMessage>> {
    let message: &String = &notif.message;
    if message.len() > 1024 {
        return Err(BasicMessage::new(400, "failed: message too long".to_string()));
    };

    match notif.mode.as_str() {
        "info" => match info(&notif.message).await {
            Ok(_) => Ok(BasicMessage::new(200, "ok".to_string())),
            Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
        },
        "err" => match err(&notif.message).await {
            Ok(_) => Ok(BasicMessage::new(200, "ok".to_string())),
            Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
        },
        _ => Err(BasicMessage::new(400, "failed: bad mode".to_string())),
    }
}
