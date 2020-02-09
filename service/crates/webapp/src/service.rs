use rocket::{local::Client, Rocket};
use rocket_cors::CorsOptions;
use std::boxed::Box;
use tracing::debug;

pub struct Service {
    rocket: Rocket,
}

impl Service {
    pub fn new(database_url: &str, port: Option<u16>, migration_files: &str) -> Self {
        debug!("Building Universe...");

        let database = universe_database::builder::new(database_url, migration_files).unwrap();

        let healthchecker = crate::health::HealthcheckerBuilder::default()
            .add("database", Box::new(database.clone()))
            .build();

        let mut config = rocket::Config::active().unwrap();
        if let Some(port_number) = port {
            config.port = port_number;
        }

        let cors: CorsOptions = Default::default();
        let rocket = rocket::custom(config)
            .attach(cors.to_cors().unwrap())
            .attach(crate::request_id::RequestIdFairing {})
            .manage(healthchecker)
            .manage(Box::new(universe_users::new_user_service(database))
                as Box<dyn universe_users::UserService>)
            .mount("/", crate::health::routes())
            .mount("/", crate::users::routes());

        Service { rocket }
    }

    pub fn launch(self) {
        self.rocket.launch();
    }

    pub fn client(self) -> Client {
        Client::new(self.rocket).unwrap()
    }
}
