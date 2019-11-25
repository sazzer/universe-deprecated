use serde_json::{to_value, Value};
use std::result::Result;
use tera::{compile_templates, Context, Error, Tera};

/// The means to render a template into a string for sending to a client.
///
/// This will allow for arbitrary binds to be provided to the template, and will also add a special
/// function `t` to the template rendering context that allows for string translations to occur.
pub struct TemplateRenderer {
    tera: Tera,
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
    /// TemplateRenderer::new("templates/**/*");
    /// ```
    pub fn new<S: Into<&'static str>>(templates: S) -> TemplateRenderer {
        let mut tera = compile_templates!(templates.into());
        tera.autoescape_on(vec!["html"]);

        TemplateRenderer { tera: tera }
    }

    /// Render a template with a set of bind values.
    ///
    /// # Arguments
    /// * `template` The name of the template, which must exist as a file found by the glob passed to `new()`
    /// * `context` The context to pass to the template ready for it to be rendered.
    ///
    /// # Returns:
    /// The results of rendering the template
    ///
    /// # Examples
    /// ```
    /// # use universe::server::TemplateRenderer;
    /// # use tera::Context;
    /// let renderer = TemplateRenderer::new("templates/**/*");
    /// renderer.render("index.html", Context::new());
    /// ```
    ///
    /// # To-dos:
    /// * TODO: Error handling
    /// * TODO: i18n support.
    pub fn render(&self, template: &str, context: Context) -> Result<String, Error> {
        let mut tera = Tera::default();
        tera.register_function(
            "t",
            Box::new(move |_args| -> tera::Result<Value> {
                let value = to_value("Translated").unwrap();
                Ok(value)
            }),
        );
        tera.extend(&self.tera).unwrap();

        tera.render(template, &context)
    }
}
