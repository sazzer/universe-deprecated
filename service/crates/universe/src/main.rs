use dotenv::dotenv;
use tracing::{debug, info};

mod settings;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let settings = settings::Settings::new();
    debug!("Universe settings: {:?}", settings);

    universe_webapp::Service::new();
    info!("Starting Universe");
}
