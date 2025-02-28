use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum NetworkMessage {
    Block(String), // Serialized block data
    Transaction(String), // Serialized transaction data
    PeerList(Vec<String>), // List of known peers
}
