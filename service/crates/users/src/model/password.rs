use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use tracing::warn;

/// Representation of a password that has been securely hashed
#[derive(Debug, PartialEq, Clone, FromSql)]
pub struct Password(String);

/// Enumeration of errors that can occur when hashing a password
#[derive(Debug, PartialEq)]
pub enum PasswordHashError {
    Blank,
    HashError,
}

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error hashing password")
    }
}

impl std::error::Error for PasswordHashError {}

impl From<BcryptError> for PasswordHashError {
    fn from(e: BcryptError) -> Self {
        warn!("Failed to hash password: {}", e);
        PasswordHashError::HashError
    }
}

impl Password {
    /// Construct a new Password instance from an already hashed string.
    ///
    /// # Arguments
    /// * `hash` The hash to wrap
    ///
    /// # Returns
    /// The `Password` to wrap the hash
    #[allow(unused)]
    pub fn from_hash<S>(hash: S) -> Password
    where
        S: Into<String>,
    {
        Password(hash.into())
    }

    /// Generate a new Password instance from an unhashed password
    ///
    /// # Arguments
    /// * `plaintext` The password to hash
    ///
    /// # Returns
    /// The hashed `Password` representing the provided plaintext
    #[allow(unused)]
    pub fn from_plaintext<S>(plaintext: S) -> Result<Password, PasswordHashError>
    where
        S: Into<String>,
    {
        let plaintext = plaintext.into();
        if plaintext.is_empty() {
            Err(PasswordHashError::Blank)
        } else {
            let hashed = hash(plaintext, DEFAULT_COST)?;
            Ok(Password(hashed))
        }
    }

    /// Verify if our hashed password is consistent with the provided plaintext.
    ///
    /// # Arguments
    /// * `plaintext` The plaintext to compare against
    ///
    /// # returns
    /// True if the provided plaintext is consistent. False if not.
    #[allow(unused)]
    pub fn verify<S>(&self, plaintext: S) -> bool
    where
        S: Into<String>,
    {
        match verify(plaintext.into(), &self.0) {
            Ok(v) => v,
            Err(e) => {
                warn!("Error verifying password: {}", e);
                false
            }
        }
    }
}

/// Allow us to pass `Password` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for Password {
    fn to_sql(
        &self,
        t: &Type,
        w: &mut BytesMut,
    ) -> Result<IsNull, Box<dyn std::error::Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(VARCHAR, TEXT);
    to_sql_checked!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use test_env_log::test;

    #[test]
    fn test_hashing_simple_password() {
        let password = Password::from_plaintext("password");

        assert_that(&password)
            .is_ok()
            .is_not_equal_to(Password::from_hash("password"));
    }

    #[test]
    fn test_hashing_invalid_password() {
        let plaintext = std::str::from_utf8(&[65u8, 66u8, 0u8, 67u8, 68u8]).unwrap();
        let password = Password::from_plaintext(plaintext);

        assert_that(&password)
            .is_err()
            .is_equal_to(PasswordHashError::HashError);
    }

    #[test]
    fn test_hashing_blank_password() {
        let password = Password::from_plaintext("");

        assert_that(&password)
            .is_err()
            .is_equal_to(PasswordHashError::Blank);
    }

    #[test]
    fn test_verify_valid_password() {
        let password = Password::from_plaintext("password").unwrap();
        let result = password.verify("password");
        assert_that(&result).is_equal_to(true);
    }

    #[test]
    fn test_verify_wrong_password() {
        let password = Password::from_plaintext("password").unwrap();
        let result = password.verify("wrong");
        assert_that(&result).is_equal_to(false);
    }

    #[test]
    fn test_verify_wrong_case() {
        let password = Password::from_plaintext("password").unwrap();
        let result = password.verify("Password");
        assert_that(&result).is_equal_to(false);
    }

    #[test]
    fn test_verify_invalid_hash() {
        let password = Password::from_hash("password_hash");
        let result = password.verify("password_hash");
        assert_that(&result).is_equal_to(false);
    }
}
