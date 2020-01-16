use crate::{messages::Messages, TemplateRenderer};

/// Create the new Template Renderer fully ready to use
pub fn new<S>(messages_dir: S, templates_dir: S) -> TemplateRenderer
where
    S: Into<String>,
{
    let messages = Messages::new(messages_dir.into(), "en".to_owned()).unwrap();

    TemplateRenderer::new(templates_dir, messages)
}
