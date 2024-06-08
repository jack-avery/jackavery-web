use crate::endpoints::BasicMessage;

use rsourcequery::info::{ServerInfo, query};

use lazy_static::lazy_static;

use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use std::sync::Arc;
use tokio::sync::Mutex;

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use std::error::Error;
use std::time::Duration;

#[derive(Clone, Deserialize)]
pub struct HostsConfig {
    pub enabled: bool,
    pub hosts: Vec<HostConfig>,
    pub refresh_rate: u64,
    pub timeout_dur: u64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HostInfo {
    ip: String,
    network: String,
    region: String,
    players: u8,
    maxplayers: u8,
    hostname: String,
    map: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {
    pub ip: String,
    pub network: String,
    pub region: String,
    crits: bool,
    spread: bool
}

#[derive(Debug, Clone, Serialize)]
pub struct HostsEndpointResponse {
    hosts: Vec<HostInfo>,
}

lazy_static! {
    // cached host info
    static ref HOST_INFO: Arc<Mutex<Vec<HostInfo>>> = Arc::new(Mutex::new(Vec::new()));
}

pub async fn init(config: HostsConfig) {
    // load refresh rate
    let refresh_rate: Duration = Duration::from_secs(config.refresh_rate);
    let timeout_dur: Duration = Duration::from_secs(config.timeout_dur);

    // copy config into mutable environment
    tokio::task::spawn(async move {
        loop {
            dbg!("hosts: refreshing");

            let mut new_hosts_info: Vec<HostInfo> = Vec::new();
            let old_hosts_info: Vec<HostInfo> = HOST_INFO.lock().await.clone();

            for it in config.hosts.iter().zip_longest(old_hosts_info) {
                match it {
                    Both(host, old_info) => {
                        match refresh_host(host, &timeout_dur).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(err) => {
                                dbg!("{}", err);
                                new_hosts_info.push(old_info)
                            }
                        };
                    }
                    Left(host) => {
                        match refresh_host(host, &timeout_dur).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(_) => println!("getting info for {} failed", host.ip),
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
            std::mem::drop(hosts_info); // manually unlock

            dbg!("hosts: refreshed");
            tokio::time::sleep(refresh_rate).await;
        }
    });
}

async fn refresh_host<'a>(
    host: &HostConfig,
    timeout_dur: &Duration
) -> Result<HostInfo, Box<dyn Error + 'a>> {
    let info: ServerInfo = query(&host.ip, Some(*timeout_dur)).await?;

    Ok(HostInfo {
        ip: host.ip.clone(),
        network: host.network.clone(),
        region: host.region.clone(),
        players: info.players,
        maxplayers: info.maxplayers,
        hostname: info.hostname,
        map: info.map,
    })
}

pub async fn get_host_info<'a>() -> Result<HostsEndpointResponse, Box<dyn Error + 'a>> {
    Ok(HostsEndpointResponse{
        hosts: HOST_INFO.lock().await.clone()
    })
}

#[get("/hosts")]
pub async fn get_hosts() -> Result<Json<HostsEndpointResponse>, Json<BasicMessage>> {
    match get_host_info().await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
    }
}
