#![feature(proc_macro_hygiene, decl_macro)]

mod authentication;
mod health;
mod problem;
mod request_id;
mod service;
mod users;

pub use service::Service;
