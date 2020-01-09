use super::ServiceCreationError;
use crate::server::webapp::templates::{Messages, TemplateRenderer};

/// Create the new Template Renderer fully ready to use
pub fn new() -> Result<TemplateRenderer, ServiceCreationError> {
    let messages = Messages::new("messages/**/*.ftl", "en").unwrap();

    let renderer = TemplateRenderer::new("templates/**/*.tera", messages);
    Ok(renderer)
}
