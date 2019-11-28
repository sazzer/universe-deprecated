#![feature(proc_macro_hygiene, decl_macro)]

pub mod home;
pub mod login;
pub mod server;

use log::info;
use server::Server;

pub fn start() -> Server {
    info!("Building Server");

    Server::default()
        .with_routes(home::httpadapter::routes())
        .with_routes(login::httpadapter::routes())
}
