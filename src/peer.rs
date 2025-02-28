use std::net::SocketAddr;

#[derive(Debug, Clone)]
pub struct Peer {
    pub id: String,
    pub address: SocketAddr,
    pub last_seen: u64, // Timestamp of last activity
}

impl Peer {
    pub fn new(id: String, address: SocketAddr) -> Self {
        Self {
            id,
            address,
            last_seen: 0,
        }
    }

    pub fn update_last_seen(&mut self, timestamp: u64) {
        self.last_seen = timestamp;
    }
}
