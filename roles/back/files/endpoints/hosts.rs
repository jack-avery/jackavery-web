use crate::endpoints::BasicMessage;
use crate::error::WebsiteError;

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
    region: Region,
    has_community_maps: bool,
    players: u8,
    maxplayers: u8,
    hostname: String,
    map: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostConfig {
    ip: String,
    network: String,
    region: Region,
    has_community_maps: bool,
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Region {
    NAWest,
    NACentral,
    NAEast,
    SouthAmerica,
    EUWest,
    EUNorthEast,
    Asia,
    Oceania
}

impl Region {
    pub fn prepare(self, extend: bool) -> Vec<Region> {
        if extend {
            match self {
                Region::NAWest => vec![Region::NAWest, Region::NACentral, Region::NAEast],
                Region::NACentral => vec![Region::NAWest, Region::NACentral, Region::NAEast],
                Region::NAEast => vec![Region::NAWest, Region::NACentral, Region::NAEast],
                Region::SouthAmerica => vec![Region::SouthAmerica, Region::NACentral],
                Region::EUWest => vec![Region::EUWest, Region::EUNorthEast],
                Region::EUNorthEast => vec![Region::EUWest, Region::EUNorthEast],
                Region::Asia => vec![Region::Asia, Region::Oceania],
                Region::Oceania => vec![Region::Oceania, Region::Asia],
            }
        } else {
            match self {
                Region::NAWest => vec![Region::NAWest, Region::NACentral],
                Region::NACentral => vec![Region::NAWest, Region::NACentral, Region::NAEast],
                Region::NAEast => vec![Region::NACentral, Region::NAEast],
                Region::SouthAmerica => vec![Region::SouthAmerica],
                Region::EUWest => vec![Region::EUWest],
                Region::EUNorthEast => vec![Region::EUNorthEast],
                Region::Asia => vec![Region::Asia],
                Region::Oceania => vec![Region::Oceania],
            }
        }
    }
}

impl TryInto<Region> for &str {
    type Error = WebsiteError;

    fn try_into(self) -> Result<Region, Self::Error> {
        match self {
            "naw" => Ok(Region::NAWest),
            "nac" => Ok(Region::NACentral),
            "nae" => Ok(Region::NAEast),
            "sa" => Ok(Region::SouthAmerica),
            "euw" => Ok(Region::EUWest),
            "eune" => Ok(Region::EUNorthEast),
            "as" => Ok(Region::Asia),
            "oce" => Ok(Region::Oceania),
            _ => Err(WebsiteError::BadRegion(self.to_owned()))
        }
    }
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
        has_community_maps: host.has_community_maps,
        players: info.players,
        maxplayers: info.maxplayers,
        hostname: info.hostname,
        map: info.map,
    })
}

pub async fn get_host_info<'a>(
    regions: Vec<Region>,
    networks: Vec<&str>,
    allow_community_maps: bool,
    allow_empty: bool
) -> Result<HostsEndpointResponse, Box<dyn Error + 'a>> {
    let host_info: Vec<HostInfo> = HOST_INFO.lock().await.clone();
    let hosts: Vec<HostInfo> = host_info.iter()
        .filter(|h|
            regions.contains(&h.region) &&
            networks.contains(&h.network.as_str()) &&
            (!h.has_community_maps || allow_community_maps) &&
            (h.players != 0 || allow_empty)
        ).cloned().collect_vec();

    Ok(HostsEndpointResponse{
        hosts
    })
}

#[get("/hosts/<region_str>/<extend>/<allow_community_maps>/<allow_empty>/<networks_str>")]
pub async fn get_hosts(
    region_str: &str,
    extend: bool,
    allow_community_maps: bool,
    allow_empty: bool,
    networks_str: &str
) -> Result<Json<HostsEndpointResponse>, Json<BasicMessage>> {
    let region: Result<Region, WebsiteError> = region_str.try_into();
    if region.is_err() {
        return Err(BasicMessage::new(400, format!("{}", region.unwrap_err())));
    }
    let region: Region = region.unwrap();
    let regions: Vec<Region> = region.prepare(extend);

    if networks_str.len() > 128 {
        return Err(BasicMessage::new(400, "networks string too long".to_string()));
    }
    let networks: Vec<&str> = networks_str.split(':').collect();

    match get_host_info(
        regions,
        networks,
        allow_community_maps,
        allow_empty
    ).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
    }
}
