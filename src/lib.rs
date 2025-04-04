#[cfg(feature = "chrono")]
use chrono::{DateTime, FixedOffset};
use mac_address::MacAddress;
use serde_derive::{Deserialize, Serialize};
use std::net::Ipv6Addr;

#[derive(Deserialize, Serialize, Debug, Default, Clone)]
pub struct NodesJSON {
    pub version: u32,
    #[cfg(not(feature = "chrono"))]
    pub timestamp: String,
    #[cfg(feature = "chrono")]
    pub timestamp: DateTime<FixedOffset>,
    pub nodes: Vec<Node>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Node {
    pub firstseen: String,
    pub lastseen: String,
    pub flags: Flags,
    pub statistics: Statistics,
    pub nodeinfo: Nodeinfo,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Flags {
    pub online: bool,
    pub gateway: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Processes {
    pub total: u32,
    pub running: u32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Statistics {
    pub node_id: String,
    pub clients: u32,
    pub rootfs_usage: Option<f32>,
    pub memory_usage: Option<f32>,
    pub uptime: f32,
    pub idletime: Option<f32>,
    pub gateway: Option<MacAddress>,
    pub gateway6: Option<MacAddress>,
    pub processes: Processes,
    pub traffic: Traffic,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Groups {
    pub backbone: Backbone,
    pub groups: Option<u8>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Backbone {
    pub peers: Peers,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Peers {
    pub sn01: Option<Peer>,
    pub sn02: Option<Peer>,
    pub sn03: Option<Peer>,
    pub sn04: Option<Peer>,
    pub sn05: Option<Peer>,
    pub sn06: Option<Peer>,
    pub sn07: Option<Peer>,
    pub sn08: Option<Peer>,
    pub sn09: Option<Peer>,
    pub sn10: Option<Peer>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Peer {
    pub established: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Traffic {
    pub tx: Traffic0,
    pub rx: Traffic0,
    pub forward: Traffic0,
    pub mgmt_tx: Traffic0,
    pub mgmt_rx: Traffic0,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Traffic0 {
    pub bytes: Option<u64>,
    pub dropped: Option<u64>,
    pub packets: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Nodeinfo {
    pub node_id: String,
    pub network: Network,
    pub owner: Option<Owner>,
    pub hostname: String,
    pub location: Option<Location>,
    pub software: Software,
    pub hardware: Hardware,
    pub vpn: bool,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Network {
    pub mac: MacAddress,
    pub addresses: Vec<Ipv6Addr>,
    pub mesh: Option<Mesh>,
    pub mesh_interfaces: (),
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Owner {
    pub contact: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct System {
    pub site_code: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Location {
    pub longitude: f64,
    pub latitude: f64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Software {
    pub autoupdater: Option<Autoupdater>,
    #[serde(rename = "batman-adv")]
    pub batman_adv: BatmanAdv,
    pub firmware: Firmware,
    pub wireguard: Option<Wireguard>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Autoupdater {
    pub branch: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BatmanAdv {
    pub version: Option<String>,
    pub compat: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Firmware {
    pub base: Option<String>,
    pub release: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Wireguard {
    pub version: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Hardware {
    pub nproc: u32,
    pub model: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Mesh {
    pub bat0: Option<Bat0>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Bat0 {
    pub interfaces: Interfaces,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Interfaces {
    pub other: Option<Vec<MacAddress>>,
    pub wireless: Option<Vec<MacAddress>>,
    pub tunnel: Option<Vec<MacAddress>>,
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "chrono")]
    use crate::NodesJSON;

    #[test]
    #[cfg(feature = "chrono")]
    fn test_deserialize_event() {
        let json_data = r#"{
            "version": 1,
            "timestamp": "2025-03-28T20:18:28+0100",
            "nodes": []
        }"#;
        let nodes_json: NodesJSON = serde_json::from_str(json_data).expect("Failed to deserialize");
        assert_eq!(
            nodes_json.timestamp.to_string(),
            "2025-03-28 20:18:28 +01:00"
        );
    }
}
