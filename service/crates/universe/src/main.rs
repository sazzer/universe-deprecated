use dotenv::dotenv;
use tracing::{debug, info};

mod settings;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let service_base = std::fs::canonicalize(".").unwrap();
    let migrations_path = service_base.join("migrations");
    let migrations_glob = format!("{}/**/*.sql", migrations_path.to_str().unwrap());

    let settings = settings::Settings::new();
    debug!("Universe settings: {:?}", settings);

    universe_webapp::Service::new(&settings.database_url, &migrations_glob);
    info!("Starting Universe");
}
