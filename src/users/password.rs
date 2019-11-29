/// Representation of a securely hashed password that we can compare to other passwords to see if
/// they match
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    /// Construct a new `Password` from the already hashed string - i.e. when loading from the data
    /// store.
    ///
    /// # Arguments
    /// * `hash` The hash that we are wrapping in a `Password`
    ///
    /// # Returns
    /// The `Password` wrapper around the hash
    pub fn from_hash<S: Into<String>>(hash: S) -> Self {
        Password(hash.into())
    }
}
