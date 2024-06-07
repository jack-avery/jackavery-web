use crate::endpoints::BasicMessage;

use lazy_static::lazy_static;
use regex::Regex;
use rocket::serde::json::Json;
use serde::Serialize;
use serde_yaml;
use sourcon::client::Client;

use futures::lock::Mutex;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Serialize)]
struct HostInfo {
    players: usize,
    maxplayers: usize,
    connect: String,
    hostname: String,
    map: String,
    is_pass_protected: bool,
}

struct HostSettings {
    ip: String,
    port: i64,
    rcon_pass: String,
    include_password: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct HostEndpointResponse {
    age: u64,
    hosts: Vec<HostInfo>,
}

lazy_static! {
    // cached host info
    static ref HOST_INFO: Arc<Mutex<Vec<HostInfo>>> = {
        #[allow(unused_mut)] // compiler whines otherwise but we need this mutable
        let mut m: Vec<HostInfo> = Vec::new();
        Arc::new(Mutex::new(m))
    };
    // time since last refresh (for "age")
    static ref LAST_REFRESH: Arc<Mutex<Instant>> = {
        #[allow(unused_mut)] // compiler whines otherwise but we need this mutable
        let mut i: Instant = Instant::now();
        Arc::new(Mutex::new(i))
    };

    // regexes to scrape resulting status information
    // TODO: include all info, and not just the stuff i want
    static ref HOSTNAME_RE: Regex = Regex::new(r"hostname: (.+)").unwrap();
    static ref MAP_RE: Regex = Regex::new(r"map     : (.+) at").unwrap();
    static ref PLAYERS_RE: Regex = Regex::new(r"players : (\d+) humans, (\d+) bots \((\d+)").unwrap();
    static ref PASSWORD_RE: Regex = Regex::new(r#"sv_password" = "([^\"]+)""#).unwrap();
}

pub async fn init(config: &serde_yaml::Value) {
    let rate: u64 = match config.get("refresh") {
        Some(v) => v.as_u64().unwrap_or(10),
        None => 10,
    };
    let mut hosts: Vec<HostSettings> = Vec::new();
    for item in config["hosts"].as_sequence().unwrap() {
        let host = item.as_mapping().unwrap();
        hosts.push(HostSettings {
            ip: host.get("ip").unwrap().as_str().unwrap().to_string(),
            port: host.get("port").unwrap().as_i64().unwrap(),
            rcon_pass: host.get("rcon_pass").unwrap().as_str().unwrap().to_string(),
            include_password: host.get("include_password").unwrap().as_bool().unwrap(),
        });
    }
    tokio::task::spawn(async move {
        loop {
            // wait REFRESH_RATE seconds between refreshes
            tokio::time::sleep(Duration::from_secs(rate)).await;
            dbg!("hosts: refreshing");

            // create a new vec to populate
            let mut new_hosts_info: Vec<HostInfo> = Vec::new();
            let hosts_info = HOST_INFO.lock().await.clone();

            for it in hosts.iter().zip_longest(hosts_info) {
                match it {
                    Both(h, old_info) => {
                        match refresh_host(&h.ip, &h.port, &h.rcon_pass, &h.include_password).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(err) => {
                                dbg!(err);
                                new_hosts_info.push(old_info)
                            }
                        };
                    }
                    Left(h) => {
                        match refresh_host(&h.ip, &h.port, &h.rcon_pass, &h.include_password).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(_) => println!("getting info for {} failed", &h.ip),
                        };
                    }
                    Right(old_host) => {
                        // this should never happen?
                        new_hosts_info.push(old_host)
                    }
                }
            }

            let mut hosts_info = HOST_INFO.lock().await;
            *hosts_info = new_hosts_info;

            let mut last_refresh = LAST_REFRESH.lock().await;
            *last_refresh = Instant::now();

            dbg!("hosts: refreshed");
        }
    });
}

async fn refresh_host<'a>(
    host: &str,
    port: &i64,
    rcon_pass: &str,
    include_password: &bool,
) -> Result<HostInfo, Box<dyn Error + 'a>> {
    let mut connect = format!("{}:{}", host, port);

    let mut c = Client::connect(&connect, rcon_pass).await?;
    let s = c.command("status").await?;
    let status = s.body();
    let p = c.command("sv_password").await?;

    let is_pass_protected: bool;

    let password = &PASSWORD_RE.captures(p.body());
    match password {
        Some(m) => {
            if *include_password {
                connect = format!("{}; password {}", connect, &m[1]);
            }
            is_pass_protected = true;
        }
        None => {
            is_pass_protected = false;
        }
    };
    connect = format!("connect {}", connect);

    let players: usize;
    let maxplayers: usize;
    let player_re = &PLAYERS_RE.captures(status);
    match player_re {
        Some(m) => {
            players = m[1].parse::<usize>().unwrap();
            let bots = m[2].parse::<usize>().unwrap();
            let _maxplayers = m[3].parse::<usize>().unwrap();

            if bots > 0 {
                maxplayers = _maxplayers - bots;
            } else {
                maxplayers = _maxplayers;
            }
        }
        None => {
            // failed to get player info? assume empty server?
            players = 0;
            maxplayers = 24;
        }
    }

    let hostname_re = &HOSTNAME_RE.captures(status);
    let hostname: String = match hostname_re {
        Some(m) => m[1].to_string(),
        None => "could not resolve".to_string(),
    };

    let map_re = &MAP_RE.captures(status);
    let map: String = match map_re {
        Some(m) => m[1].to_string(),
        None => "could not resolve".to_string(),
    };

    Ok(HostInfo {
        players,
        maxplayers,
        connect,
        hostname,
        map,
        is_pass_protected,
    })
}

pub async fn get_host_info<'a>() -> Result<HostEndpointResponse, Box<dyn Error + 'a>> {
    // return current value
    let hostinfo = HOST_INFO.lock().await.clone();

    let last_refresh = *LAST_REFRESH.lock().await;
    let age = Instant::now().duration_since(last_refresh).as_secs();

    Ok(HostEndpointResponse {
        age,
        hosts: hostinfo,
    })
}

#[get("/hosts")]
pub async fn get_hosts() -> Result<Json<HostEndpointResponse>, Json<BasicMessage>> {
    match get_host_info().await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
    }
}
