use dotenv;

fn main() {
    dotenv::dotenv().ok();
    env_logger::init();

    universe::start().run();
}
