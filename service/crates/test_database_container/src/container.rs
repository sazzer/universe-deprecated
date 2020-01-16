use lazy_static::lazy_static;
use postgres::{Client, NoTls};
use testcontainers::{clients::Cli, images::postgres::Postgres, Container, Docker};
use tracing::info;

lazy_static! {
    static ref DOCKER: Cli = { Cli::default() };
}

/// Wrapper around a Docker container that runs Postgres for our tests
pub struct TestDatabase<'d> {
    #[allow(dead_code)]
    node: Container<'d, Cli, Postgres>,
    pub host: String,
    pub port: u32,
    pub url: String,
}

impl<'d> TestDatabase<'d> {
    pub fn new() -> Self {
        let node = DOCKER.run(Postgres::default());

        let host = std::env::var("DOCKER_HOSTNAME").unwrap_or("localhost".to_owned());
        let port = node.get_host_port(5432).unwrap();
        let url = format!("postgres://postgres:postgres@{}:{}", host, port);
        info!("Running postgres on {}", url);

        TestDatabase {
            node,
            host,
            port,
            url,
        }
    }

    pub fn client(&self) -> Client {
        Client::connect(&self.url, NoTls).unwrap()
    }
}
