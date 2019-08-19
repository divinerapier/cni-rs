use serde::{Deserialize, Serialize};

use crate::libcni::types::DNS;

const IMPLEMENTED_SPEC_VERSION: &'static str = "1.0.0";

#[derive(Serialize, Deserialize)]
pub struct Interface {
    pub name: String,
    pub mac: String,
    pub sandbox: String,
}

#[derive(Serialize, Deserialize)]
pub struct IPConfig {
    #[serde(rename = "Interface")]
    pub interface: Option<usize>,
    #[serde(rename = "Address")]
    pub address: ipnetwork::IpNetwork,
    #[serde(rename = "Gateway")]
    pub gateway: std::net::IpAddr,
}

#[derive(Serialize, Deserialize)]
pub struct Result {
    pub cni_version: String,
    pub interfaces: Vec<Interface>,
    pub ips: Vec<IPConfig>,
    pub routes: Vec<super::super::types::Route>,
    pub dns: super::super::types::DNS,
}
