use super::Messages;
use serde_json::{to_value, Value};
use std::collections::HashMap;
use std::result::Result;
use std::sync::Arc;
use tera::{compile_templates, Context, Error, Tera};

/// The means to render a template into a string for sending to a client.
///
/// This will allow for arbitrary binds to be provided to the template, and will also add a special
/// function `t` to the template rendering context that allows for string translations to occur.
pub struct TemplateRenderer {
    tera: Tera,
    messages: Arc<Messages>,
}

impl TemplateRenderer {
    /// Construct a new instance of the `TemplateRenderer` ready for use.
    ///
    /// # Arguments
    /// * `templates` The glob defining where the templates should be loaded from
    ///
    /// # Examples
    /// ```
    /// # use universe::server::TemplateRenderer;
    /// TemplateRenderer::new("templates/**/*", "messages", "en");
    /// ```
    pub fn new<S: Into<&'static str>>(
        templates: S,
        messages: S,
        default_locale: S,
    ) -> TemplateRenderer {
        let mut tera = compile_templates!(templates.into());
        tera.autoescape_on(vec![]);

        let messages = Messages::new(messages.into(), default_locale.into());

        TemplateRenderer {
            tera: tera,
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
    /// # Examples
    /// ```
    /// # use universe::server::TemplateRenderer;
    /// # use tera::Context;
    /// let renderer = TemplateRenderer::new("templates/**/*", "messages", "en");
    /// renderer.render("index.html", vec![], Context::new());
    /// ```
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

        let messages = self.messages.clone();
        tera.register_function(
            "t",
            Box::new(move |args: HashMap<String, Value>| -> tera::Result<Value> {
                let params: HashMap<&str, Value> = args
                    .iter()
                    .map(|(key, value)| (key.as_ref(), value.clone()))
                    .collect();

                let message = args
                    .get("key")
                    .and_then(|v| v.as_str())
                    .map(|key| messages.lookup(locales.clone(), key.to_owned(), params))
                    .map(|value| to_value(value).unwrap());

                match message {
                    Some(m) => Ok(m),
                    None => Ok(to_value("!!!!!Oops!!!!!").unwrap()),
                }
            }),
        );
        tera.extend(&self.tera).unwrap();

        let _ = tera.full_reload();

        tera.render(template, &context)
    }
}
