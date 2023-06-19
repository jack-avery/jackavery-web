use crate::endpoints::{gen_error, ErrorMessage};

use serde::Serialize;
use regex::Regex;
use rocket::serde::json::Json;
use lazy_static::lazy_static;
use serde_yaml;
use sourcon::client::Client;

use std::sync::{Arc, Mutex};
use std::error::Error;
use std::time::Duration;

#[derive(Debug, Clone, Serialize)]
pub struct HostInfo {
    players: u8,
    maxplayers: u8,
    connect: String,
    hostname: String,
    map: String,
    is_pass_protected: bool,
}

pub struct HostSettings {
    ip: String,
    port: i64,
    rcon_pass: String,
    include_password: bool,
}

lazy_static! {
    // cached host info
    static ref HOST_INFO: Arc<Mutex<Vec<HostInfo>>> = {
        #[allow(unused_mut)] // compiler whines otherwise but we need this mutable
        let mut m: Vec<HostInfo> = Vec::new();
        Arc::new(Mutex::new(m))
    };

    // regexes to scrape resulting status information
    // TODO: include all info, and not just the stuff i want
    static ref HOSTNAME_RE: Regex = Regex::new(r"hostname: (.+)").unwrap();
    static ref MAP_RE: Regex = Regex::new(r"map     : (.+) at").unwrap();
    static ref PLAYERS_RE: Regex = Regex::new(r"players : (\d+) humans, 0 bots \((\d+)").unwrap();
    // idk if there's a better way to match everything except "
    static ref PASSWORD_RE: Regex = Regex::new(r#"sv_password" = "([\w\d`~!@#$%^&*()\-_=+,<.>/?;:'\[{\]}\\| ]+)""#).unwrap();
}

pub async fn init(config: serde_yaml::Value) {
    // parse config
    // TODO: probably make this better. probably.
    let rate: u64 = config["endpoints"]["hosts"]["refresh"].as_u64().unwrap();
    let mut hosts: Vec<HostSettings> = Vec::new();
    for item in config["endpoints"]["hosts"]["hosts"].as_sequence().unwrap() {
        let host = item.as_mapping().unwrap();
        hosts.push( HostSettings {
            ip: host.get("ip").unwrap().as_str().unwrap().to_string(),
            port: host.get("port").unwrap().as_i64().unwrap(),
            rcon_pass: host.get("rcon_pass").unwrap().as_str().unwrap().to_string(),
            include_password: host.get("include_password").unwrap().as_bool().unwrap(),
        });
    }
    tokio::task::spawn(
        async move {
            loop {
                // wait REFRESH_RATE seconds between refreshes
                tokio::time::sleep(Duration::from_secs(rate)).await;
                dbg!("hosts: refreshing");

                // create a new vec to populate
                let mut new_hosts_info: Vec<HostInfo> = Vec::new();
                for h in hosts.iter() {
                    match refresh_host(&h.ip, &h.port, &h.rcon_pass, &h.include_password).await {
                        Ok(info) => new_hosts_info.push(info),
                        // TODO: use old data instead of giving up??
                        Err(_) => println!("getting info for {} failed", &h.ip),
                    };
                }

                // replace hosts_info with new info
                let mut hosts_info = HOST_INFO.lock().unwrap();
                *hosts_info = new_hosts_info;
                dbg!("hosts: refreshed");
            }
        }
    );
}

pub async fn refresh_host<'a>(host: &str, port: &i64, rcon_pass: &str, include_password: &bool) -> Result<HostInfo, Box<dyn Error + 'a>> {
    let mut connect = format!("{}:{}", host, port);

    let mut c = Client::connect(&connect, &rcon_pass).await?;
    let s = c.command("status").await?;
    let status = s.body();
    let p = c.command("sv_password").await?;

    let is_pass_protected: bool;
    let captures = &PASSWORD_RE.captures(p.body());

    match captures {
        Some(r) => {
            if *include_password {
                connect = format!("{}; password {}", connect, &r[1]);
            }
            is_pass_protected = true;
        },
        None => {
            is_pass_protected = false;
        },
    };
    connect = format!("connect {}", connect);

    Ok(HostInfo {
        players: PLAYERS_RE.captures(&status).unwrap()[1].parse::<u8>().unwrap(),
        maxplayers: PLAYERS_RE.captures(&status).unwrap()[2].parse::<u8>().unwrap(),
        connect,
        hostname: HOSTNAME_RE.captures(&status).unwrap()[1].to_string(),
        map: MAP_RE.captures(&status).unwrap()[1].to_string(),
        is_pass_protected,
    })
}

pub async fn get_host_info<'a>() -> Result<Vec<HostInfo>, Box<dyn Error + 'a>> {
    // return current value
    let hostinfo = HOST_INFO.lock().unwrap().clone();
    Ok(hostinfo)
}

#[get("/hosts")]
pub async fn get_hosts() -> Result<Json<Vec<HostInfo>>, Json<ErrorMessage>> {
    match get_host_info().await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(gen_error(500, format!("failed: {}", e.to_string())))
    }
}
