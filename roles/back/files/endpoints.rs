use rocket::serde::json::Json;
use serde::Serialize;
use serde_yaml;

pub mod hosts;

#[derive(Debug, Clone, Serialize)]
pub struct ErrorMessage {
    code: u16,
    message: String,
}

pub fn gen_error(code: u16, message: String) -> Json<ErrorMessage> {
    Json(ErrorMessage { code, message })
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
}