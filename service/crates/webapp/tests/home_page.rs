use service::{assert_response, ServiceWrapper};

mod service;

#[test]
fn test_render_home_page() {
    tracing_subscriber::fmt::Builder::default()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    let service = ServiceWrapper::new();

    let req = service.client.get("/");
    let response = req.dispatch();
    assert_response("test_render_home_page", response);
}
