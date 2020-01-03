use serde_json::{to_value, Value};
use std::collections::HashMap;
use tera::{Context, Error, Tera};

/// The means to render a template into a string for sending to a client.
///
/// This will allow for arbitrary binds to be provided to the template, and will also add a special
/// function `t` to the template rendering context that allows for string translations to occur.
pub struct TemplateRenderer {
    tera: Tera,
}

/// Tera function for looking up a message key
struct MessageLookup {}

impl tera::Function for MessageLookup {
    /// Look up the message key identified by the argument "key", providing any other values to
    /// it as needed.
    ///
    /// This will work according to the requested locale of the client, will fallback to the default
    /// as appropriate
    fn call(&self, args: &HashMap<String, Value>) -> Result<Value, Error> {
        let message: Option<String> = None;

        let message_key = args
            .get("key")
            .and_then(|v| v.as_str())
            .unwrap_or("no-key-present");

        let value = match message {
            Some(m) => m,
            None => format!("!!!!!{}!!!!!", message_key),
        };

        to_value(value).map_err(|_e| Error::msg("Oops"))
    }
}

impl TemplateRenderer {
    /// Construct a new instance of the `TemplateRenderer` ready for use.
    ///
    /// # Arguments
    /// * `templates` The glob defining where the templates should be loaded from
    pub fn new<S>(templates: S) -> TemplateRenderer
    where
        S: Into<String>,
    {
        let tera = Tera::new(templates.into().as_str()).unwrap();
        TemplateRenderer { tera }
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
        _locales: Vec<String>,
        context: Context,
    ) -> Result<String, Error> {
        let mut tera = Tera::default();

        let message_lookup = MessageLookup {};
        tera.register_function("t", message_lookup);

        tera.extend(&self.tera)?;

        tera.render(template, &context)
    }
}
