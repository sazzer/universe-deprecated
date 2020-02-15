use dotenv::dotenv;
use tracing::{debug, info};

mod settings;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let service_base = std::fs::canonicalize(".").unwrap();
    let migrations_path = service_base.join("migrations");
    let migrations_glob = format!("{}/**/*.sql", migrations_path.to_str().unwrap());

    let settings = settings::Settings::new();
    debug!("Universe settings: {:?}", settings);

    let service =
        universe_webapp::Service::new(&settings.database_url, settings.port, &migrations_glob);
    info!("Starting Universe");
    service.launch();
}
