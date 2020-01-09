use insta::assert_snapshot;
use lazy_static::lazy_static;
use rocket::local::{Client, LocalResponse};
use std::fmt::Write;
use testcontainers::{clients::Cli, images::postgres::Postgres, Container, Docker};
use tracing::info;
use universe::Service;

lazy_static! {
    static ref DOCKER: Cli = { Cli::default() };
}

#[allow(dead_code)]
pub struct ServiceWrapper<'d> {
    node: Container<'d, Cli, Postgres>,
    url: String,
    pub client: Client,
}

impl<'d> ServiceWrapper<'d> {
    pub fn new() -> Self {
        info!("Creating service");

        let node = DOCKER.run(Postgres::default());

        let host = std::env::var("DOCKER_HOSTNAME").unwrap_or("localhost".to_owned());
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres:postgres@{}:{}", host, port);

        let service = Service::new(url.clone(), Some(0)).unwrap();

        ServiceWrapper {
            node,
            url,
            client: service.client(),
        }
    }
}

pub fn assert_response(name: &'static str, mut response: LocalResponse) {
    let mut output = String::new();
    writeln!(output, "{}", response.status()).unwrap();

    let mut headers = response.headers().clone();
    headers.remove("X-Request-ID");
    for header in headers.iter() {
        writeln!(output, "{}", header).unwrap();
    }

    writeln!(output, "").unwrap();
    writeln!(
        output,
        "{}",
        response.body_string().unwrap_or("".to_owned())
    )
    .unwrap();
    assert_snapshot!(name, output);
}
