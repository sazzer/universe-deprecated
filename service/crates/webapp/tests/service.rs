use insta::assert_snapshot;
use rocket::local::{Client, LocalResponse};
use std::fmt::Write;
use tracing::info;
use universe::Service;
use universe_test_database_wrapper::TestDatabaseWrapper;

#[allow(dead_code)]
pub struct ServiceWrapper<'d> {
    pub database: TestDatabaseWrapper<'d>,
    pub client: Client,
}

impl<'d> ServiceWrapper<'d> {
    pub fn new() -> Self {
        info!("Creating service");

        let database = TestDatabaseWrapper::new();

        let service_base = std::fs::canonicalize("../..").unwrap();
        let service = Service::new(
            database.url(),
            Some(0),
            service_base.to_str().unwrap().to_owned(),
        )
        .unwrap();

        ServiceWrapper {
            database,
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
