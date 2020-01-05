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

    let service = Service::new(settings.database_url, settings.port).unwrap();

    service.launch();
}
