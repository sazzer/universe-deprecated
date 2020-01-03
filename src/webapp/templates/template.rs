use super::renderer::TemplateRenderer;
use accept_language::parse;
use log::{debug, error};
use rocket::{
    http::{ContentType, Status},
    response::{Responder, Result},
    Request, Response, State,
};
use serde::Serialize;
use std::io::Cursor;
use tera::Context;

/// Representation of a template that we want to render so that we can send it to a client.
///
/// This contains the name of the template and any data inserts that it needs to render correctly.
pub struct Template {
    name: String,
    data: Context,
}

impl Template {
    /// Create a new template ready to be sent to the client.
    ///
    /// # Arguments
    /// `template` The name of the template to render
    pub fn new<S: Into<String>>(template: S) -> Template {
        Template {
            name: template.into(),
            data: Context::new(),
        }
    }

    /// Add a new piece of data that the template needs to work with.
    ///
    /// # Arguments
    /// `key` The key for the piece of data, as expected inside the template
    /// `val` The value for the piece of data
    pub fn with_data<T: Serialize + ?Sized, S: Into<String>>(
        mut self,
        key: S,
        val: &T,
    ) -> Template {
        self.data.insert(&key.into(), val);
        self
    }

    /// Get the name of the template that is to be rendered
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    /// Get the entire set of data to bind to the template that is to be rendered
    pub fn get_data(&self) -> Context {
        self.data.clone()
    }
}

/// Implementation of the `Responder` trait from Rocket so that a Template is able to render itself
/// for sending to a client.
///
/// This depends on a `TemplateRenderer` being available as managed state for the request being
/// processed.
impl<'r> Responder<'r> for Template {
    /// Respond to a request from which the controller returned a `Template` instance by rendering
    /// the template and sending the generated output down to the client.
    ///
    /// This will always send a sized body, and always has a Content-Type of `text/html` and a status
    // code of "200 OK". At present no effort is put into other headers, and specifically the Caching
    /// ones.
    ///
    /// # Arguments:
    /// * `request` The request that this template is being rendered for. This gives access to any
    ///   managed state for the request, specifically the `TemplateRenderer`, and also allows us to
    ///   get the locale to render in for i18n purposes.
    ///
    /// # Returns:
    /// The result of rendering the template, or an error if the template could not be rendered
    /// correctly.
    ///
    /// # To-dos:
    /// * TODO: Error handling
    fn respond_to(self, request: &Request) -> Result<'r> {
        let renderer: State<TemplateRenderer> = request.guard().succeeded().ok_or_else(|| {
            error!("Template Renderer not available");
            Status::InternalServerError
        })?;

        let locales = request
            .headers()
            .get_one("accept-language")
            .map(|locales| parse(locales))
            .unwrap_or(vec![]);
        debug!(
            "Rendering template {} with locales {:?}",
            self.name, locales
        );

        let rendered = renderer
            .render(&self.name, locales, self.data)
            .map_err(|e| {
                error!("Failed to render template: {:?}", e);
                Status::InternalServerError
            })?;

        Response::build()
            .sized_body(Cursor::new(rendered))
            .header(ContentType::with_params(
                "text",
                "html",
                ("charset", "utf-8"),
            ))
            .ok()
    }
}
