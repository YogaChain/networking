use crate::message::NetworkMessage;

pub struct Gossip {
    messages: Vec<NetworkMessage>,
}

impl Gossip {
    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub fn propagate(&self, message: NetworkMessage) {
        // Implement gossip propagation logic
    }
}
