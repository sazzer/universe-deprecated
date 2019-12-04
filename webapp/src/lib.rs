#![feature(proc_macro_hygiene, decl_macro)]

pub mod database;
pub mod entity;
pub mod home;
pub mod login;
pub mod server;
pub mod users;

use database::migrate::MigratableDatabase;
use log::info;
use server::Server;
use std::boxed::Box;
use std::sync::Arc;

pub fn build() -> Result<Server, String> {
    info!("Building Server");

    let database = Arc::new(
        database::postgres::PostgresDatabase::new(
            "postgres://universe:universe@localhost:45432/universe",
        )
        .map_err(|e| format!("{:?}", e))?,
    );
    database
        .migrate("migrations")
        .map_err(|_| "Failed to migrate database")?;

    let user_repository = Box::new(users::postgres_repository::PostgresUserRepository {});

    let user_service: Arc<dyn users::UserService> =
        Arc::new(users::service::UserServiceImpl::new(user_repository));

    let server = Server::new("templates/**/*", "messages", "en")
        .map_err(|_| "Failed to create server")?
        .with_service(user_service)
        .with_routes(home::httpadapter::routes())
        .with_routes(login::httpadapter::routes());

    Ok(server)
}
