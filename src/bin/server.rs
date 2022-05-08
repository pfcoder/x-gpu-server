use tracing::Level;
use tracing_subscriber::FmtSubscriber;
use xgpu_server::server;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    server().await;
}
