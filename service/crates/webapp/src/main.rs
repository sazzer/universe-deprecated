use dotenv::dotenv;
use tracing_subscriber::filter::EnvFilter;
use universe::Service;

mod settings;

fn main() {
    dotenv().ok();

    tracing_subscriber::fmt::Builder::default()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let settings = settings::Settings::new().unwrap();

    let service_base = std::fs::canonicalize(".").unwrap();
    let service = Service::new(
        settings.database_url,
        settings.port,
        service_base.to_str().unwrap().to_owned(),
    )
    .unwrap();

    service.launch();
}
