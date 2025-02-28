pub struct NetworkConfig {
    pub max_peers: usize,
    pub listen_address: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            max_peers: 100,
            listen_address: "0.0.0.0:30303".to_string(),
        }
    }
}
