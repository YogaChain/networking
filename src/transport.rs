use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub struct Transport;

impl Transport {
    pub async fn start_listener(address: &str) -> tokio::io::Result<()> {
        let listener = TcpListener::bind(address).await?;
        println!("Listening on {}", address);

        while let Ok((mut socket, _)) = listener.accept().await {
            tokio::spawn(async move {
                let mut buffer = [0; 1024];
                let _ = socket.read(&mut buffer).await;
                let _ = socket.write_all(b"Response").await;
            });
        }

        Ok(())
    }
}
