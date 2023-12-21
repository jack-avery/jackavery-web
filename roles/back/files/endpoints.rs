use rocket::serde::json::Json;
use serde::Serialize;

pub mod hosts;
pub mod rasbot;

#[derive(Debug, Clone, Serialize)]
pub struct BasicMessage {
    code: u16,
    message: String,
}

impl BasicMessage {
    fn new(code: u16, message: String) -> Json<BasicMessage> {
        Json(BasicMessage { code, message })
    }
}

//

pub async fn init() {
    let cfg_file = match std::fs::File::open("config.yml") {
        Ok(file) => file,
        Err(_) => panic!("missing config.yml"),
    };

    let cfg: serde_yaml::Value = match serde_yaml::from_reader(cfg_file) {
        Ok(cfg) => cfg,
        Err(_) => panic!("could not parse config.yml"),
    };

    if cfg["endpoints"]["hosts"]
        .get("enabled")
        .unwrap_or(&serde_yaml::Value::Bool(false))
        .as_bool()
        .unwrap_or(false)
    {
        hosts::init(&cfg["endpoints"]["hosts"]).await;
    }
    if cfg["endpoints"]["rasbot"]
        .get("enabled")
        .unwrap_or(&serde_yaml::Value::Bool(false))
        .as_bool()
        .unwrap_or(false)
    {
        rasbot::init(&cfg["endpoints"]["rasbot"]).await;
    }
}
