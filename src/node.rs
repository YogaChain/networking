use crate::network_config::NetworkConfig;
use crate::peer::Peer;
use std::net::SocketAddr;

pub struct Node {
    pub id: String,
    pub address: SocketAddr,
    pub config: NetworkConfig,
}

impl Node {
    pub fn new(id: String, address: SocketAddr, config: NetworkConfig) -> Self {
        Self { id, address, config }
    }

    pub fn connect_peer(&self, peer: &Peer) {
        println!("Connecting to peer: {:?}", peer);
    }
}
