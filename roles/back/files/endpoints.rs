use rocket::serde::json::Json;
use serde::Serialize;
use serde_yaml;

pub mod hosts;
pub mod rasbot;

#[derive(Debug, Clone, Serialize)]
pub struct BasicMessage {
    code: u16,
    message: String,
}

pub fn gen_msg(code: u16, message: String) -> Json<BasicMessage> {
    Json(BasicMessage { code, message })
}

//

pub async fn init() {
    let cfg_file = match std::fs::File::open("config.yml") {
        Ok(file) => file,
        Err(_) => panic!("missing config.yml")
    };

    let cfg: serde_yaml::Value = match serde_yaml::from_reader(cfg_file) {
        Ok(cfg) => cfg,
        Err(_) => panic!("could not parse config.yml")
    };

    hosts::init(cfg.clone()).await;
    rasbot::init(cfg.clone()).await;
}