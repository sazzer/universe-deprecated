use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use log::{debug, error};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use unic_langid::LanguageIdentifier;

/// Type abbreviation for a bundle of messages for a single locale
type Bundle = FluentBundle<FluentResource>;

/// Wrapper around all of the messages that we have loaded, covering all locales
pub struct Messages {
    default_locale: LanguageIdentifier,
    bundles: Vec<Bundle>,
}

impl Messages {
    /// Construct a new `Messages` struct.
    ///
    /// This loaded every file from the `messages` directory, using the filename to indicate the
    /// locale that it represents. The one specified by `default_locale` is the one that we will
    /// use as the default when actually looking up a message
    ///
    /// # Arguments
    /// * `messages` Directory containing the message files to load
    /// * `default_locale` The locale to treat as the default when looking up messages
    ///
    /// # Returns
    /// A `Messages` instance that we can use for looking up messages
    ///
    /// # Examples
    /// ```
    /// # use universe::server::Messages;
    /// Messages::new("./messages", "en");
    /// ```
    pub fn new<S: Into<String>>(messages: S, default_locale: S) -> Self {
        let mut bundles: Vec<Bundle> = Vec::new();

        let files = fs::read_dir(messages.into()).unwrap();
        for entry in files {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }

            debug!("Processing file: {:?}", entry);

            let path = entry.path();
            let locale: LanguageIdentifier = path
                .file_stem()
                .expect("file on disk has no name")
                .to_str()
                .unwrap()
                .parse()
                .unwrap();
            debug!("Locale for file {:?}: {}", entry, locale);

            let source = fs::read_to_string(&path).unwrap();
            let resource = FluentResource::try_new(source).unwrap();

            let mut bundle = FluentBundle::new(&[locale]);
            bundle.add_resource(resource).unwrap();

            bundles.push(bundle);
        }

        Messages {
            bundles: bundles,
            default_locale: default_locale.into().parse().unwrap(),
        }
    }

    pub fn full_reload(&mut self) {}

    /// Find the message bundle that contains the specified message key and is the best match for
    /// the list of locales provided.
    ///
    /// This returns the bundle that is the best match for the earliest locale in the list.
    ///
    /// For example, if we have bundles of "en", "fr", "fr_FR" and "fr_CA", where "en" is the default:
    /// * Requesting ["fr_FR"] will match bundle "fr_FR"
    /// * Requesting ["fr_CH"] will match bundle "fr"
    /// * Requesting ["es_ES"] will match the default bundle "en"
    /// * Requesting ["fr_CH", "fr_CA"] will match bundle "fr_CA"
    /// * Requesting ["es_ES", "fr_CH"] will match bundle "fr"
    ///
    /// # Arguments
    /// * `locales` The list of locales to match against. Typically this is the parsed `Accept-Language`
    ///   header from an incoming HTTP request
    /// * `message_key` The message key to look up
    ///
    /// # Returns
    /// The bundle that matches our request, or `None` if we couldn't find one.
    fn find_bundle(&self, locales: &Vec<LanguageIdentifier>, message_key: &str) -> Option<&Bundle> {
        // Filter down to all the bundles containing our message
        let bundles = &self.bundles;
        let bundles_with_message: Vec<&Bundle> = bundles
            .into_iter()
            .filter(|bundle| bundle.has_message(message_key))
            .collect();

        // Build the actual list of locales to match against, including the default
        let mut desired_locales: Vec<&LanguageIdentifier> = Vec::new();
        for locale in locales.iter() {
            desired_locales.push(&locale);
        }
        desired_locales.push(&self.default_locale);

        // Iterate over each Locale, trying to find a bundle that matches
        // This looks for an exact match first, and then a partial match otherwise
        for locale in desired_locales.iter() {
            for bundle in bundles_with_message.iter() {
                // Check every bundle for an exact match first
                for bundle_locale in bundle.locales.iter() {
                    if locale.matches(bundle_locale, false, false) {
                        return Some(bundle);
                    }
                }
            }
            for bundle in bundles_with_message.iter() {
                // Failing that, try again looking for partial matches.
                for bundle_locale in bundle.locales.iter() {
                    if locale.matches(bundle_locale, false, true) {
                        return Some(bundle);
                    }
                }
            }
        }

        // If we get here then we didn't find a match at all
        None
    }

    /// Look up the message for the provided message key in the provided locales.
    ///
    /// This returns the message that is the best match for the earliest locale in the list.
    ///
    /// For example, if we have bundles of "en", "fr", "fr_FR" and "fr_CA", where "en" is the default:
    /// * Requesting ["fr_FR"] will match bundle "fr_FR"
    /// * Requesting ["fr_CH"] will match bundle "fr"
    /// * Requesting ["es_ES"] will match the default bundle "en"
    /// * Requesting ["fr_CH", "fr_CA"] will match bundle "fr_CA"
    /// * Requesting ["es_ES", "fr_CH"] will match bundle "fr"
    ///
    /// # Arguments
    /// * `locales` The list of locales to match against. Typically this is the parsed `Accept-Language`
    ///   header from an incoming HTTP request
    /// * `message_key` The message key to look up
    ///
    /// # Returns
    /// The translated message. If we couldn't find a message that matches, or performing the translation
    /// fails for any reason when instead we return a special message `!!!{message_key}!!!`
    ///
    /// # Examples
    /// ## Successful translation
    /// ```
    /// # use universe::server::Messages;
    /// # use std::collections::HashMap;
    /// let messages = Messages::new("src/server/test_messages", "en");
    /// let message = messages.lookup(vec!["en"], "hello", HashMap::new());
    /// assert_eq!("world", message);
    /// ```
    /// ## Fallback to default locale
    /// ```
    /// # use universe::server::Messages;
    /// # use std::collections::HashMap;
    /// let messages = Messages::new("src/server/test_messages", "en");
    /// let message = messages.lookup(vec!["es_ES"], "hello", HashMap::new());
    /// assert_eq!("world", message);
    /// ```
    /// ## Unknown message key
    /// ```
    /// # use universe::server::Messages;
    /// # use std::collections::HashMap;
    /// let messages = Messages::new("src/server/test_messages", "en");
    /// let message = messages.lookup(vec!["en"], "unknown", HashMap::new());
    /// assert_eq!("!!!unknown!!!", message);
    /// ```
    pub fn lookup<S: Into<String>>(
        &self,
        locales: Vec<S>,
        message_key: S,
        params: HashMap<&str, Value>,
    ) -> String {
        let desired_message_key: String = message_key.into();
        let desired_locales: Vec<LanguageIdentifier> = locales
            .into_iter()
            .map(|locale| locale.into())
            .map(|locale| locale.parse().unwrap())
            .collect();
        debug!(
            "Looking up message key '{}' for locales '{:?}'",
            desired_message_key, &desired_locales
        );

        match self.find_bundle(&desired_locales, &desired_message_key) {
            None => {
                error!(
                    "No bundle found for message key '{}' and locales '{:?}'",
                    desired_message_key, desired_locales
                );
                format!("!!!{}!!!", desired_message_key)
            }
            Some(bundle) => {
                debug!(
                    "Found bundle found for message key '{}' and locales '{:?}'",
                    desired_message_key, desired_locales
                );

                bundle
                    .get_message(&desired_message_key)
                    .and_then(|message| message.value)
                    .and_then(|pattern| {
                        let mut args = FluentArgs::new();
                        for (key, value) in params {
                            let fluent_value = match value {
                                Value::Number(n) => FluentValue::into_number(n),
                                Value::String(s) => FluentValue::into_number(s),
                                _ => FluentValue::None,
                            };

                            args.insert(key, fluent_value);
                        }

                        let mut errors = vec![];
                        let result = bundle.format_pattern(&pattern, Some(&args), &mut errors);

                        if errors.is_empty() {
                            Some(result.into_owned())
                        } else {
                            error!(
                                "Error formatting message for message key '{}' and locales '{:?}': {:?}",
                                desired_message_key, desired_locales, errors
                            );
                            None
                        }
                    })
                    .unwrap_or(format!("!!!{}!!!", desired_message_key))
            }
        }
    }
}

// TODO: Tests for variable interpolation
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_new_with_messages() {
        Messages::new("src/server/test_messages", "en");
    }

    #[test]
    #[should_panic]
    fn test_new_with_no_messages() {
        Messages::new("missing", "en");
    }

    #[test]
    fn test_format_simple() {
        let messages = Messages::new("src/server/test_messages", "en");
        let formatted = messages.lookup(vec!["en"], "hello", HashMap::new());
        assert_eq!("world", formatted);
    }

    #[test]
    fn test_format_not_default() {
        let messages = Messages::new("src/server/test_messages", "en");
        let formatted = messages.lookup(vec!["fr"], "hello", HashMap::new());
        assert_eq!("Bonjour", formatted);
    }

    #[test]
    fn test_format_fallback_to_default() {
        let messages = Messages::new("src/server/test_messages", "en");
        let formatted = messages.lookup(vec!["fr"], "answer", HashMap::new());
        assert_eq!("42", formatted);
    }

    #[test]
    fn test_format_first_locale() {
        let messages = Messages::new("src/server/test_messages", "en");
        let formatted = messages.lookup(vec!["fr_CA", "fr"], "hello", HashMap::new());
        assert_eq!("Bonjour!", formatted);
    }

    #[test]
    fn test_format_fallback_to_second_locale() {
        let messages = Messages::new("src/server/test_messages", "en");
        let formatted = messages.lookup(vec!["fr_CA", "fr"], "goodbye", HashMap::new());
        assert_eq!("Au revoir", formatted);
    }
}
