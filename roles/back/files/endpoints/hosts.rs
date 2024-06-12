use crate::endpoints::BasicMessage;
use crate::error::WebsiteError;

use rsourcequery::info::{query_timeout_duration, ServerInfo};

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
    crits: bool,
    spread: bool,
    alltalk: bool,
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
    spread: bool,
    alltalk: bool
}

#[derive(Debug, Clone, Serialize)]
pub struct HostsEndpointResponse {
    code: u16,
    matches: usize,
    hosts: Vec<HostInfo>,
}

lazy_static! {
    // cached host info
    static ref HOST_INFO: Arc<Mutex<Vec<HostInfo>>> = Arc::new(Mutex::new(Vec::new()));
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Preference {
    None,
    Enabled,
    Disabled,
}

impl From<u8> for Preference {
    fn from(value: u8) -> Self {
        match value {
            0u8 => Self::Disabled,
            1u8 => Self::Enabled,
            _ => Self::None
        }
    }
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
            println!("hosts: refreshing");

            let mut new_hosts_info: Vec<HostInfo> = Vec::new();
            let old_hosts_info: Vec<HostInfo> = HOST_INFO.lock().await.clone();

            for it in config.hosts.iter().zip_longest(old_hosts_info) {
                match it {
                    Both(host, old_info) => {
                        match refresh_host(host, &timeout_dur).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(err) => {
                                eprintln!("getting info for {} failed: {}", host.ip, err);
                                new_hosts_info.push(old_info)
                            }
                        };
                    }
                    Left(host) => {
                        match refresh_host(host, &timeout_dur).await
                        {
                            Ok(info) => new_hosts_info.push(info),
                            Err(err) => eprintln!("getting info for {} failed: {}", host.ip, err),
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

            println!("hosts: refreshed");
            tokio::time::sleep(refresh_rate).await;
        }
    });
}

async fn refresh_host<'a>(
    host: &HostConfig,
    timeout_dur: &Duration
) -> Result<HostInfo, Box<dyn Error + 'a>> {
    let info: ServerInfo = query_timeout_duration(&host.ip, *timeout_dur).await?;

    Ok(HostInfo {
        ip: host.ip.clone(),
        network: host.network.clone(),
        region: host.region.clone(),
        has_community_maps: host.has_community_maps,
        crits: host.crits,
        spread: host.spread,
        alltalk: host.alltalk,
        players: info.players,
        maxplayers: info.maxplayers,
        hostname: info.hostname,
        map: info.map,
    })
}

#[allow(clippy::too_many_arguments)]
pub async fn get_host_info<'a>(
    regions: Vec<Region>,
    networks: Vec<&str>,
    allow_empty: bool,
    allow_full: bool,
    allow_community_maps: bool,
    crits_pref: Preference,
    spread_pref: Preference,
    alltalk_pref: Preference
) -> Result<HostsEndpointResponse, Box<dyn Error + 'a>> {
    let host_info: Vec<HostInfo> = HOST_INFO.lock().await.clone();
    let hosts: Vec<HostInfo> = host_info.iter()
        .filter(|h|
            regions.contains(&h.region) && // region
            networks.contains(&h.network.as_str()) && // network
            (!h.has_community_maps || allow_community_maps) && // maps
            (h.players > 0 || allow_empty) && // empty
            (h.players < h.maxplayers || allow_full) && // full
            (
                match crits_pref {
                    Preference::None => true, // no pref
                    Preference::Enabled => h.crits, // yes random crits
                    Preference::Disabled => !h.crits, // no random crits
                }
            ) &&
            (
                match spread_pref {
                    Preference::None => true, // no pref
                    Preference::Enabled => h.spread, // yes random spread
                    Preference::Disabled => !h.spread, // no random spread
                }
            ) &&
            (
                match alltalk_pref {
                    Preference::None => true, // no pref
                    Preference::Enabled => h.alltalk, // yes alltalk
                    Preference::Disabled => !h.alltalk, // no alltalk
                }
            )
        ).cloned().collect_vec();

    Ok(HostsEndpointResponse{
        code: 200,
        matches: hosts.len(),
        hosts
    })
}

#[allow(clippy::too_many_arguments)]
#[get("/hosts/<regions_str>/<networks_str>/<allow_empty>/<allow_full>/<allow_community_maps>/<crits_pref>/<spread_pref>/<alltalk_pref>")]
pub async fn get_hosts(
    regions_str: &str,
    allow_empty: bool,
    allow_full: bool,
    allow_community_maps: bool,
    crits_pref: u8,
    spread_pref: u8,
    alltalk_pref: u8,
    networks_str: &str
) -> Result<Json<HostsEndpointResponse>, Json<BasicMessage>> {
    if regions_str.len() > 32 {
        return Err(BasicMessage::new(400, "regions string too long".to_string()));
    }
    let regions_list: Vec<&str> = regions_str.split(':').collect();

    let mut regions: Vec<Region> = Vec::new();
    for region in regions_list {
        let region: Result<Region, WebsiteError> = region.try_into();
        if region.is_err() {
            return Err(BasicMessage::new(400, format!("{}", region.unwrap_err())));
        }
        regions.push(region.unwrap());
    }

    if networks_str.len() > 128 {
        return Err(BasicMessage::new(400, "networks string too long".to_string()));
    }
    let networks: Vec<&str> = networks_str.split(':').collect();

    match get_host_info(
        regions,
        networks,
        allow_empty,
        allow_full,
        allow_community_maps,
        crits_pref.into(),
        spread_pref.into(),
        alltalk_pref.into()
    ).await {
        Ok(r) => Ok(Json(r)),
        Err(e) => Err(BasicMessage::new(500, format!("failed: {}", e))),
    }
}
