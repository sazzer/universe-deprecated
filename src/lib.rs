#![feature(proc_macro_hygiene, decl_macro)]

pub mod home;
pub mod server;

use log::info;
use rocket::routes;
use server::Server;

pub fn start() -> Server {
    info!("Building Server");

    Server::default().with_routes(routes![home::httpadapter::index])
}
