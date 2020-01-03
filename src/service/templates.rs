use super::ServiceCreationError;
use crate::webapp::templates::TemplateRenderer;

/// Create the new Template Renderer fully ready to use
pub fn new() -> Result<TemplateRenderer, ServiceCreationError> {
    let renderer = TemplateRenderer::new("templates/**/*.tera");
    Ok(renderer)
}
