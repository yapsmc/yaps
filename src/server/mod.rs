use log::info;
use tokio::{io::AsyncReadExt, net::TcpListener};

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    pub async fn new() -> Self {
        let listener = TcpListener::bind("0.0.0.0:25565")
            .await
            .expect("Failed to start server");

        Self { listener }
    }

    pub async fn start(self) {
        loop {
            let (mut socket, client_addr) = self.listener.accept().await.unwrap();

            info!("client connected: {:?}", client_addr);

            tokio::spawn(async move {
                let mut buf = Vec::new();

                socket.read_to_end(&mut buf).await.unwrap();
                info!("{:?}", buf);
            });
        }
    }
}
