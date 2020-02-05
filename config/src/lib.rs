use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    version_major: u32,
    version_minor: u32,
    coin_name: String,
    node_drop_off_threshold: u64, // percent of online nodes that can go offline before a rearrange
    decimal_places: u8,
    max_connections: u16,
    max_threads: u8,
    chain_key: String,
    state: u8,
    host: u64,
    seednodes: Vec<Vec<u8>>,
    ignore_minor_updates: bool,
    p2p_port: u16,
    rpc_port: u16,
    allow_cors: String,
}  

pub fn config() -> Config {
  let mut file = File::open("node.conf").unwrap();
    let mut data: String = "";
    file.read_to_string(&mut data).unwrap();
    let conf: Config = serde_json::from_str(&data).unwrap();
    return conf;
}
