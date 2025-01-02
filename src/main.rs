use tokio;

mod api;
mod game; // Declare the game module
use api::api::start_server;
use tracing_subscriber::fmt::Subscriber;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // if let Ok(level) = std::env::var("RUST_LOG") {
    //     tracing_subscriber::fmt()
    //         .with_env_filter(EnvFilter::new(&format!(
    //             "{}={level}",
    //             env!("CARGO_PKG_NAME").replace("-", "_"),
    //         )))
    //         .init();
    // }

    // Start the server
    start_server().await;
}
