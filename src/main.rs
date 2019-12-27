use dotenv::dotenv;
use universe::Service;

fn main() {
    dotenv().ok();
    env_logger::init();

    Service::new("postgres://universe:universe@localhost:45432/universe").unwrap();
}
