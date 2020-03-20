use rocket::{local::Client, Rocket};
use rocket_cors::CorsOptions;
use std::boxed::Box;
use tracing::debug;

pub struct Service {
    rocket: Rocket,
}

impl Service {
    pub fn new(
        database_url: &str,
        port: Option<u16>,
        access_token_key: &str,
        migration_files: &str,
    ) -> Self {
        debug!("Building Universe...");

        let database = universe_database::builder::new(database_url, migration_files).unwrap();

        let healthchecker = crate::health::HealthcheckerBuilder::default()
            .add("database", Box::new(database.clone()))
            .build();

        let mut config = rocket::Config::active().unwrap();
        if let Some(port_number) = port {
            config.port = port_number;
        }

        let cors: CorsOptions = CorsOptions {
            allow_credentials: true,
            expose_headers: ["link".to_owned()].iter().cloned().collect(),
            max_age: Some(3600),
            ..Default::default()
        };
        let rocket = rocket::custom(config)
            .attach(cors.to_cors().unwrap())
            .attach(crate::request_id::RequestIdFairing {})
            .manage(healthchecker)
            .manage(universe_authentication::AccessTokenFactory::new(
                chrono::Duration::days(365),
            ))
            .manage(universe_authentication::encoder::AccessTokenEncoder::new(
                access_token_key,
            ))
            // TODO: Stop cloning the database wrapper
            .manage(Box::new(universe_users::new_user_service(database.clone()))
                as Box<dyn universe_users::UserService>)
            .manage(
                Box::new(universe_worlds::new_world_service(database.clone()))
                    as Box<dyn universe_worlds::WorldService>,
            )
            .mount("/", crate::health::routes())
            .mount("/", crate::users::routes())
            .mount("/", crate::worlds::routes())
            .mount("/", crate::authentication::routes());

        Service { rocket }
    }

    pub fn launch(self) {
        self.rocket.launch();
    }

    pub fn client(self) -> Client {
        Client::new(self.rocket).unwrap()
    }
}
