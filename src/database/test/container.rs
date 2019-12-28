use lazy_static::lazy_static;
use log::info;
use postgres::{Client, NoTls};
use testcontainers::{clients::Cli, images::postgres::Postgres, Container, Docker};

lazy_static! {
    static ref DOCKER: Cli = { Cli::default() };
}

pub struct TestDatabase<'d> {
    #[allow(dead_code)]
    node: Container<'d, Cli, Postgres>,
    pub port: u32,
    pub url: String,
}

impl<'d> TestDatabase<'d> {
    pub fn new() -> Self {
        let node = DOCKER.run(Postgres::default());

        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres:postgres@localhost:{}", port);
        info!("Running postgres on {}", url);

        TestDatabase { node, port, url }
    }

    pub fn client(&self) -> Client {
        Client::connect(&self.url, NoTls).unwrap()
    }
}
