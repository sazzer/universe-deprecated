#![feature(proc_macro_hygiene, decl_macro)]

pub mod server;

use log::info;
use rocket::{get, routes};
use server::{Messages, Template, TemplateRenderer};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/t")]
fn templated() -> Template {
    Template::new("index.html").with_data("name", "Graham")
}

pub fn start() {
    info!("Starting Server");

    let template_renderer = TemplateRenderer::new("templates/**/*");
    let messages = Messages::new("messages", "en");

    rocket::ignite()
        .manage(template_renderer)
        .mount("/", routes![index, templated])
        .launch();
}
