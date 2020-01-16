use super::Messages;
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tera::{Context, Error, Tera};

/// The means to render a template into a string for sending to a client.
///
/// This will allow for arbitrary binds to be provided to the template, and will also add a special
/// function `t` to the template rendering context that allows for string translations to occur.
pub struct TemplateRenderer {
    tera: Tera,
    messages: Arc<Messages>,
}

/// Tera function for looking up a message key
struct MessageLookup {
    messages: Arc<Messages>,
    locales: Vec<String>,
}

impl tera::Function for MessageLookup {
    /// Look up the message key identified by the argument "key", providing any other values to
    /// it as needed.
    ///
    /// This will work according to the requested locale of the client, will fallback to the default
    /// as appropriate
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, Error> {
        let message_key = args
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("no-key-present");

        let value =
            self.messages
                .lookup(self.locales.clone(), message_key.to_owned(), args.clone());

        to_value(value).map_err(|_e| Error::msg("Oops"))
    }
}

impl TemplateRenderer {
    /// Construct a new instance of the `TemplateRenderer` ready for use.
    ///
    /// # Arguments
    /// * `templates` The glob defining where the templates should be loaded from
    pub fn new<S>(templates: S, messages: Messages) -> TemplateRenderer
    where
        S: Into<String>,
    {
        let tera = Tera::new(templates.into().as_str()).unwrap();
        TemplateRenderer {
            tera,
            messages: Arc::new(messages),
        }
    }

    /// Render a template with a set of bind values.
    ///
    /// # Arguments
    /// * `template` The name of the template, which must exist as a file found by the glob passed to `new()`
    /// * `locales` A list of the locales to use for rendering, in preference order
    /// * `context` The context to pass to the template ready for it to be rendered.
    ///
    /// # Returns:
    /// The results of rendering the template
    ///
    /// # To-dos:
    /// * TODO: Error handling
    pub fn render(
        &self,
        template: &str,
        locales: Vec<String>,
        context: Context,
    ) -> Result<String, Error> {
        let mut tera = Tera::default();

        let message_lookup = MessageLookup {
            messages: self.messages.clone(),
            locales,
        };
        tera.register_function("t", message_lookup);

        tera.extend(&self.tera)?;

        tera.render(template, &context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;

    #[test]
    fn test_no_templates() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/empty/**/*.tera", messages);

        let rendered = renderer.render("hello.tera", vec![], Context::new());
        assert_debug_snapshot!("renderer_no_templates", rendered);
    }

    #[test]
    fn test_simple_template() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/full/**/*.tera", messages);

        let rendered = renderer.render("simple.tera", vec![], Context::new());
        assert_debug_snapshot!("renderer_simple_template", rendered);
    }

    #[test]
    fn test_template_inserts() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/full/**/*.tera", messages);

        let mut context = Context::new();
        context.insert("name", "Graham");
        let rendered = renderer.render("inserts.tera", vec![], context);
        assert_debug_snapshot!("renderer_template_inserts", rendered);
    }

    #[test]
    fn test_i18n() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/full/**/*.tera", messages);

        let rendered = renderer.render("i18n.tera", vec![], Context::new());
        assert_debug_snapshot!("renderer_i18n", rendered);
    }

    #[test]
    fn test_i18n_override() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/full/**/*.tera", messages);

        let rendered = renderer.render("i18n.tera", vec!["en_US".to_owned()], Context::new());
        assert_debug_snapshot!("renderer_i18n_override", rendered);
    }

    #[test]
    fn test_i18n_insert() {
        let messages = Messages::new("test_messages/full/**/*.ftl", "en").unwrap();
        let renderer = TemplateRenderer::new("test_templates/full/**/*.tera", messages);

        let mut context = Context::new();
        context.insert("name", "Graham");
        let rendered = renderer.render("i18n_inserts.tera", vec![], context);
        assert_debug_snapshot!("renderer_i18n_insert", rendered);
    }
}
