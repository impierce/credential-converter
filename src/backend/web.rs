use crate::backend::routes::create_router;

use eyre::Result;

use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;

pub struct Server {
    address: IpAddr,
    port: u16,
}

impl Server {
    pub fn new(address: Option<String>) -> Self {
        // Use default address if no input is provided
        let default_address = "127.0.0.1".to_string();
        let default_port = 3000;

        // Parse the provided address and port or fall back to defaults
        let address = address.unwrap_or(default_address);
        let mut split = address.split(':');
        let ip_part = split.next().unwrap_or("127.0.0.1");
        let port_part = split.next().unwrap_or("3000");

        let address: IpAddr = IpAddr::from_str(ip_part).unwrap_or_else(|_| IpAddr::from([127, 0, 0, 1]));
        let port: u16 = port_part.parse().unwrap_or(default_port);

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        let app = create_router();
        let address = SocketAddr::from((self.address, self.port));

        tracing::info!("server running on port: {}", self.port);

        // run our app with hyper, listening globally on port 3000
        let listener = tokio::net::TcpListener::bind(address).await.unwrap();
        axum::serve(listener, app).await.unwrap();

        Ok(())
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(None)
    }
}

#[tokio::main]
pub async fn api_service(address: Option<String>) {
    let server = Server::new(address);

    match server.run().await {
        Ok(_) => println!("Server exited"),
        Err(error) => panic!("Server exited with error: {error}"),
    }
}
