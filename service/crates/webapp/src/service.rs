use rocket::{local::Client, Rocket};
use tracing::debug;

pub struct Service {
    rocket: Rocket,
}

impl Service {
    pub fn new(database_url: &str, port: Option<u16>, migration_files: &str) -> Self {
        debug!("Building Universe...");

        let database = universe_database::builder::new(database_url, migration_files).unwrap();

        let healthchecker = crate::health::HealthcheckerBuilder::default()
            .add("database", database)
            .build();

        let mut config = rocket::Config::active().unwrap();
        if let Some(port_number) = port {
            config.port = port_number;
        }

        let rocket = rocket::custom(config)
            .manage(healthchecker)
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
