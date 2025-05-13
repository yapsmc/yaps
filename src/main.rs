mod protocol;
mod server;

use log::info;
use server::Server;
use simplelog::*;

#[tokio::main]
async fn main() {
    TermLogger::init(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    );

    let server = Server::new().await;

    info!("server started");
    server.start().await;
}
