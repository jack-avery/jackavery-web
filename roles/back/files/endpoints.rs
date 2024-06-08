use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};

use crate::endpoints::{rasbot::RasbotConfig, hosts::HostsConfig};

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

#[derive(Deserialize)]
struct Config {
    rasbot: RasbotConfig,
    hosts: HostsConfig
}

//

pub async fn init() {
    let cfg_bytes = match std::fs::read("config.yml") {
        Ok(bytes) => bytes,
        Err(_) => panic!("missing config.yml"),
    };
    let cfg_str = std::str::from_utf8(&cfg_bytes)
        .expect("failed to utf8decode config");

    let cfg: Config = serde_yaml::from_str(cfg_str).expect("failed to parse config");

    if cfg.hosts.enabled {
        hosts::init(cfg.hosts.clone()).await;
    }
    if cfg.rasbot.enabled {
        rasbot::init(cfg.rasbot.clone()).await;
    }
}
