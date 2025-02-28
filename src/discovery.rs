use crate::peer::Peer;
use std::collections::HashMap;

pub struct PeerDiscovery {
    known_peers: HashMap<String, Peer>,
}

impl PeerDiscovery {
    pub fn new() -> Self {
        Self {
            known_peers: HashMap::new(),
        }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.known_peers.insert(peer.id.clone(), peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        self.known_peers.remove(peer_id);
    }

    pub fn get_known_peers(&self) -> Vec<Peer> {
        self.known_peers.values().cloned().collect()
    }
}
