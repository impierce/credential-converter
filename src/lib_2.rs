use std::net::SocketAddr;

use eyre::Result;
use routes::create_router;

mod routes;

pub struct Server {
    address: [u8; 4],
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        let address = [127, 0, 0, 1];
        let port = 3000;

        Self { address, port }
    }

    pub async fn run(&self) -> Result<()> {
        tracing_subscriber::fmt::init();

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
        Self::new()
    }
}
