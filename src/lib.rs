#![feature(proc_macro_hygiene, decl_macro)]

use log::info;
use rocket::{get, http::ContentType, response::Responder, routes, Request, Response, State};
use serde::Serialize;
use std::io::Cursor;
use tera::{compile_templates, Context, Tera};

struct TemplateRenderer {
    tera: Tera,
}

struct Template {
    name: String,
    data: Context,
}

impl Template {
    fn new<S: Into<String>>(template: S) -> Template {
        Template {
            name: template.into(),
            data: Context::new(),
        }
    }

    fn with_data<T: Serialize + ?Sized, S: Into<String>>(mut self, key: S, val: &T) -> Template {
        self.data.insert(&key.into(), val);
        self
    }
}

impl<'r> Responder<'r> for Template {
    fn respond_to(self, request: &Request) -> rocket::response::Result<'r> {
        let renderer = request.guard::<State<TemplateRenderer>>().unwrap();

        let context = self.data;

        let rendered = renderer.tera.render(self.name.as_ref(), &context).unwrap();

        Response::build()
            .sized_body(Cursor::new(rendered))
            .header(ContentType::new("text", "html"))
            .ok()
    }
}
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

    let mut tera = compile_templates!("templates/**/*");
    tera.autoescape_on(vec!["html"]);

    let template_renderer = TemplateRenderer { tera: tera };
    rocket::ignite()
        .manage(template_renderer)
        .mount("/", routes![index, templated])
        .launch();
}
