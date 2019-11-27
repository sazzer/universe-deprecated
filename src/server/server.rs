use super::TemplateRenderer;
use rocket::{Rocket, Route};
use rocket_contrib::serve::StaticFiles;

/// Representation of the server to actually work with
#[derive(Default)]
pub struct Server {
    routes: Vec<Route>,
}

impl Server {
    /// Register some routes onto the server for us to serve up
    ///
    /// # Arguments
    /// * `routes` The routes to add to the server. All routes are assumed to be pinned to the root
    ///   of the server and need to know their own paths
    ///
    /// # Returns
    /// This, to allow for a fluid interface
    pub fn with_routes(mut self, routes: Vec<Route>) -> Self {
        let mut routes = routes.clone();
        self.routes.append(&mut routes);
        self
    }

    /// Build the Rocket server that we're actually going to work with
    fn build(&self) -> Rocket {
        let template_renderer = TemplateRenderer::new("templates/**/*", "messages", "en");

        rocket::ignite()
            .manage(template_renderer)
            .mount("/static", StaticFiles::from("static"))
            .mount("/", self.routes.clone())
    }

    /// Build and launch the server
    pub fn run(&self) {
        self.build().launch();
    }
}
