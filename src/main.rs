use dotenv::dotenv;
use universe::Service;

mod settings;

fn main() {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let settings = settings::Settings::new().unwrap();

    let service = Service::new(settings.database_url, settings.port).unwrap();

    service.launch();
}