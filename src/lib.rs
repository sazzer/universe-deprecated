#![feature(proc_macro_hygiene, decl_macro)]

pub mod database;
pub mod entity;
pub mod home;
pub mod login;
pub mod server;
pub mod users;

use log::info;
use server::Server;
use std::boxed::Box;
use std::sync::Arc;

pub fn start() -> Server {
    info!("Building Server");

    let database: Arc<dyn database::Database<_>> =
        Arc::new(database::postgres::PostgresDatabase::new(
            "postgres://universe:universe@localhost:45432/universe",
        ));

    database.client().unwrap().execute("SELECT 1", &[]).unwrap();

    let user_repository = Box::new(users::postgres_repository::PostgresUserRepository {});

    let user_service: Arc<dyn users::UserService> =
        Arc::new(users::service::UserServiceImpl::new(user_repository));

    Server::default()
        .with_service(user_service)
        .with_routes(home::httpadapter::routes())
        .with_routes(login::httpadapter::routes())
}
