use rocket::{Rocket, Route};
use rocket_contrib::serve::StaticFiles;
use universe_templates::TemplateRenderer;

/// Representation of the server to actually work with
pub struct Server {
    rocket: Rocket,
}

impl Server {
    /// Construct a new HTTP server that we can start configuring
    ///
    /// # Arguments
    /// * `templates` The glob with which to find templates
    /// * `messages` The directory in which to find message bundles
    /// * `default_locale` The default locale for the message bundles
    ///
    /// # Returns
    /// The server for us to use
    pub fn new<S: Into<&'static str>>(
        templates: S,
        messages: S,
        default_locale: S,
    ) -> Result<Self, ()> {
        let template_renderer = TemplateRenderer::new(templates, messages, default_locale);

        let rocket = rocket::ignite()
            .manage(template_renderer)
            .mount("/static", StaticFiles::from("static"));

        Ok(Server { rocket: rocket })
    }

    /// Register some routes onto the server for us to serve up
    ///
    /// # Arguments
    /// * `routes` The routes to add to the server. All routes are assumed to be pinned to the root
    ///   of the server and need to know their own paths
    ///
    /// # Returns
    /// This, to allow for a fluid interface
    pub fn with_routes(self, routes: Vec<Route>) -> Self {
        Server {
            rocket: self.rocket.mount("/", routes),
        }
    }

    /// Register a new service that can be used by routes and middleware on the server
    ///
    /// # Arguments
    /// * `service` The service to register with the server so that routes and middleware can access it
    ///
    /// # Returns
    /// This, to allow for a fluid interface
    pub fn with_service<T: Send + Sync + 'static>(self, service: T) -> Self {
        Server {
            rocket: self.rocket.manage(service),
        }
    }

    /// Build the Rocket server that we're actually going to work with
    fn build(self) -> Rocket {
        self.rocket
    }

    /// Build and launch the server
    pub fn run(self) {
        self.build().launch();
    }
}
