use tokio::sync::mpsc;
use crate::peer::Peer;

pub struct P2P {
    peers: Vec<Peer>,
}

impl P2P {
    pub fn new() -> Self {
        Self { peers: Vec::new() }
    }

    pub fn add_peer(&mut self, peer: Peer) {
        self.peers.push(peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peers.retain(|peer| peer.id != peer_id);
    }
}
