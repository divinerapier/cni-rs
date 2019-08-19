use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct RuntimeConf {
    container_id: String,
    net_ns: String,
    if_name: String,
    args: Vec<[String; 2]>,
    capability_args: HashMap<String, String>,
    cache_dir: String,
}

pub struct NetworkConfig {
    pub network: NetConf,
    pub bytes: Vec<u8>,
}

// impl <T> NetworkConfig<T> {
//     pub fn new(bytes: &[u8]) ->
// }

pub struct NetworkConfigList {
    pub name: String,
    pub cni_version: String,
    pub disable_check: bool,
    pub plugins: Vec<NetworkConfig>,
    pub bytes: Vec<u8>,
}

pub struct NetConf {
    pub cni_version: String,
    pub name: String,
    pub _type: String,
    pub capabilities: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct IPAM {
    #[serde(rename = "type")]
    pub _type: String,
}

pub struct NetConfList {
    pub cni_version: String,
    pub name: String,
    pub disable_check: bool,
    pub plugins: Vec<NetworkConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct DNS {
    pub nameservers: Vec<String>,
    pub domain: String,
    pub search: Vec<String>,
    pub options: Vec<String>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Route {
    #[serde(rename = "Dst")]
    pub dst: ipnetwork::IpNetwork,
    #[serde(rename = "GW")]
    pub gw: std::net::IpAddr,
}
