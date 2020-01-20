use dotenv::dotenv;
use tracing::info;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    info!("Starting Universe");
}
