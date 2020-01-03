use fluent::{FluentArgs, FluentBundle, FluentResource, FluentValue};
use glob::glob;
use log::{debug, warn};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;
use unic_langid::LanguageIdentifier;

/// Type abbreviation for a bundle of messages for a single locale
type Bundle = FluentBundle<FluentResource>;

pub struct Messages {
    default_locale: LanguageIdentifier,
    bundles: Vec<Bundle>,
}

#[derive(Debug, PartialEq)]
pub struct MessagesError {}

impl From<unic_langid::LanguageIdentifierError> for MessagesError {
    fn from(e: unic_langid::LanguageIdentifierError) -> Self {
        warn!("Failed to parse a language identifier: {:?}", e);
        MessagesError {}
    }
}

impl From<glob::PatternError> for MessagesError {
    fn from(e: glob::PatternError) -> Self {
        warn!("Failed to parse glob for language files: {:?}", e);
        MessagesError {}
    }
}

impl Messages {
    pub fn new<S>(messages: S, default_locale: S) -> Result<Messages, MessagesError>
    where
        S: Into<String>,
    {
        let default_locale = default_locale.into().parse()?;

        let files: Vec<PathBuf> = glob(messages.into().as_str())?
            .filter_map(Result::ok)
            .collect();

        let mut bundles: Vec<Bundle> = Vec::new();

        for entry in files {
            debug!("Processing file: {:?}", entry);

            let locale: LanguageIdentifier = entry
                .file_stem()
                .and_then(|file| file.to_str())
                .ok_or(MessagesError {})?
                .parse()?;

            debug!("Locale for file {:?}: {:?}", entry, locale);

            let bundle = std::fs::read_to_string(&entry)
                .map_err(|_| "Failed to read file")
                .and_then(|source| {
                    FluentResource::try_new(source).map_err(|err| {
                        warn!("Error building resource: {:?}", err.1);
                        "Failed to build resource"
                    })
                })
                .and_then(|resource| {
                    let mut bundle = FluentBundle::new(&[locale]);
                    bundle
                        .add_resource(resource)
                        .map_err(|_| "Failed to add resource to bundle")?;
                    Ok(bundle)
                });

            match bundle {
                Ok(bundle) => {
                    debug!("Loaded bundle for file {:?}", entry);
                    bundles.push(bundle);
                }
                Err(err) => warn!("Error building bundle {:?}: {}", entry, err),
            }
        }

        Ok(Messages {
            default_locale,
            bundles,
        })
    }

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
        let mut desired_locales: Vec<&LanguageIdentifier> = locales.iter().collect();
        desired_locales.push(&self.default_locale);
        debug!("Finding bundle for desired locales: {:?}", desired_locales);

        // Iterate over each Locale, trying to find a bundle that matches
        // This looks for an exact match first, and then a partial match otherwise
        for bundle in bundles_with_message.iter() {
            for locale in desired_locales.iter() {
                // Check every bundle for an exact match first
                for bundle_locale in bundle.locales.iter() {
                    if locale.matches(bundle_locale, false, false) {
                        debug!(
                            "Matched locale {:?} against bundle {:?}",
                            locale, bundle_locale
                        );
                        return Some(bundle);
                    }
                }
            }
            for locale in desired_locales.iter() {
                // Failing that, try again looking for partial matches.
                for bundle_locale in bundle.locales.iter() {
                    if locale.matches(bundle_locale, false, true) {
                        debug!(
                            "Loosely matched locale {:?} against bundle {:?}",
                            locale, bundle_locale
                        );
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
    pub fn lookup<S>(
        &self,
        locales: Vec<S>,
        message_key: S,
        params: HashMap<String, Value>,
    ) -> String
    where
        S: Into<String>,
    {
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
                warn!(
                    "No bundle found for message key '{}' and locales '{:?}'",
                    desired_message_key, desired_locales
                );
                format!("!!!{}!!!", desired_message_key)
            }
            Some(bundle) => {
                debug!(
                    "Found bundle for message key '{}' and locales '{:?}'",
                    desired_message_key, desired_locales
                );

                bundle
                    .get_message(&desired_message_key)
                    .and_then(|message| message.value)
                    .and_then(|pattern| {
                        let mut args = FluentArgs::new();
                        for (key, value) in &params {
                            let fluent_value = match value {
                                Value::Number(n) => FluentValue::into_number(n),
                                Value::String(s) => FluentValue::into_number(s),
                                _ => FluentValue::None,
                            };

                            args.insert(&key, fluent_value);
                        }

                        let mut errors = vec![];
                        let result = bundle.format_pattern(&pattern, Some(&args), &mut errors);

                        if errors.is_empty() {
                            Some(result.into_owned())
                        } else {
                            warn!(
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
