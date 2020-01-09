use service::{assert_response, ServiceWrapper};

mod service;

#[test]
fn test_render_start_login_page() {
    let _ = tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let service = ServiceWrapper::new();

    let req = service.client.get("/login");
    let response = req.dispatch();
    assert_response("login_test_render_start_login_page", response);
}

#[test]
fn test_submit_missing_username() {
    let _ = tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let service = ServiceWrapper::new();

    let req = service
        .client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("");
    let response = req.dispatch();
    assert_response("login_test_submit_missing_username", response);
}

#[test]
fn test_submit_blank_username() {
    let _ = tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let service = ServiceWrapper::new();

    let req = service
        .client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("username=&");
    let response = req.dispatch();
    assert_response("login_test_submit_blank_username", response);
}

#[test]
fn test_submit_whitespace_username() {
    let _ = tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let service = ServiceWrapper::new();

    let req = service
        .client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("username=    &");
    let response = req.dispatch();
    assert_response("test_submit_whitespace_username", response);
}

#[test]
fn test_submit_unknown_username() {
    let _ = tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .try_init();

    let service = ServiceWrapper::new();

    let req = service
        .client
        .post("/login")
        .header(rocket::http::ContentType::Form)
        .body("username=unknown&");
    let response = req.dispatch();
    assert_response("test_submit_unknown_username", response);
}
