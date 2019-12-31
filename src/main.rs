use dotenv::dotenv;
use universe::Service;

fn main() {
    dotenv().ok();
    env_logger::init();

    let service = Service::new("postgres://universe:universe@localhost:45432/universe").unwrap();
    service.launch();
}
