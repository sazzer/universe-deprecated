use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use bytes::BytesMut;
use log::warn;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Serialize, Serializer};
use std::error::Error;

/// Representation of a password that has been securely hashed
#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

/// Enumeration of errors that can occur when hashing a password
#[derive(Debug, PartialEq)]
pub enum PasswordHashError {
    HashError,
}

impl From<BcryptError> for PasswordHashError {
    fn from(_: BcryptError) -> Self {
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
    pub fn from_plaintext<S>(plaintext: S) -> Result<Password, PasswordHashError>
    where
        S: Into<String>,
    {
        let hashed = hash(plaintext.into(), DEFAULT_COST)?;
        Ok(Password(hashed))
    }

    /// Verify if our hashed password is consistent with the provided plaintext.
    ///
    /// # Arguments
    /// * `plaintext` The plaintext to compare against
    ///
    /// # returns
    /// True if the provided plaintext is consistent. False if not.
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

/// Allow us to serialize `Password` objects into Serde structures.
///
/// This always outputs `null` regardless of the password, so that we can never accidentally leak
/// hashed passwords out to clients.
impl Serialize for Password {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_none()
    }
}

/// Allow us to pass `Password` objects to Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be used directly as database
/// binds without ever needing to extract the string from inside it.
impl ToSql for Password {
    fn to_sql(&self, t: &Type, w: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>> {
        self.0.to_sql(t, w)
    }

    accepts!(VARCHAR, TEXT);
    to_sql_checked!();
}

/// Allow us to retrieve `Password` objects from Postgres as part of executing a database query.
///
/// The implementation of this trait allows objects of this type to be read directly as database
/// outputs without needing to construct it explicitly. Instead the Postgres crate will do so for us.
impl<'a> FromSql<'a> for Password {
    fn from_sql(t: &Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>> {
        String::from_sql(t, raw).map(Password)
    }
    accepts!(VARCHAR, TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use postgres::Error;
    use spectral::prelude::*;

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

    #[test]
    fn test_serde_serialize() {
        let password = Password::from_hash("password_hash");

        let serialized = serde_json::to_value(password);
        assert_that(&serialized)
            .is_ok()
            .is_equal_to(serde_json::Value::Null);
    }

    #[test]
    fn test_postgres_to_sql() {
        let database = TestDatabase::new();

        let password = Password::from_hash("password_hash");
        let result = database.client().query("SELECT $1", &[&password]);

        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: &str = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to("password_hash");
    }

    #[test]
    fn test_postgres_from_sql_valid_type() {
        let database = TestDatabase::new();

        let result = database.client().query("SELECT $1", &[&"password_hash"]);

        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: Password = rows.get(0).unwrap().get(0);
        assert_that(&output_value).is_equal_to(Password::from_hash("password_hash"));
    }

    #[test]
    fn test_postgres_from_sql_invalid_type() {
        let database = TestDatabase::new();

        let result = database.client().query("SELECT $1", &[&1]);

        let err: Error = result.err().unwrap();
        let err_msg = format!("{}", err);
        assert_that(&err_msg)
                    .is_equal_to("error serializing parameter 0: cannot convert between the Rust type `i32` and the Postgres type `text`".to_owned());
    }
}
