use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use log::warn;

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

    /// Construct a new `Password` from the unhashed string - i.e. when taking input from a user
    ///
    /// # Arguments
    /// * `plaintext` The plaintext that we are hashing into a `Password`
    ///
    /// # Returns
    /// The `Password` wrapper around the hash
    pub fn from_plaintext<S: Into<String>>(plaintext: S) -> Result<Self, BcryptError> {
        hash(plaintext.into(), DEFAULT_COST).map(|hashed| Password(hashed))
    }

    /// Verify if the provided plaintext is consistent with this hashed password.
    ///
    /// # Arguments
    /// * `plaintext` The plaintext that we are comparing with this hash
    ///
    /// # Returns
    /// True if the passwords match. False if not.
    pub fn verify<S: Into<String>>(&self, plaintext: S) -> bool {
        match verify(plaintext.into(), &self.0) {
            Ok(true) => true,
            Ok(false) => false,
            Err(e) => {
                warn!("Error verifying hash: {}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use speculate::speculate;

    speculate! {
        before {
            let _ = env_logger::try_init();
        }

        describe "from_hash" {
            test "stores the value given" {
                let password = Password::from_hash("someHash");
                assert_eq!("someHash", password.0);
            }
        }

        describe "from_plaintext" {
            test "doesn't store the value given" {
                let password = Password::from_plaintext("someHash").unwrap();
                assert_ne!("someHash", password.0);
            }
        }

        describe "verify" {
            test "verifies against the correct password" {
                let password = Password::from_plaintext("myPassword").unwrap();
                assert!(password.verify("myPassword"));
            }

            test "doesn't verify against the wrong password" {
                let password = Password::from_plaintext("myPassword").unwrap();
                assert!(!password.verify("mypassword"));
            }

            test "doesn't verify against an invalid hash" {
                let password = Password::from_hash("mypassword");
                assert!(!password.verify("mypassword"));
            }
        }
    }
}
