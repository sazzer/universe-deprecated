use super::ServiceCreationError;
use crate::server::webapp::templates::{Messages, TemplateRenderer};

/// Create the new Template Renderer fully ready to use
pub fn new<S>(messages_dir: S, templates_dir: S) -> Result<TemplateRenderer, ServiceCreationError>
where
    S: Into<String>,
{
    let messages = Messages::new(messages_dir.into(), "en".to_owned()).unwrap();

    let renderer = TemplateRenderer::new(templates_dir, messages);
    Ok(renderer)
}
