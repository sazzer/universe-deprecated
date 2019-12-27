use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use bytes::BytesMut;
use log::warn;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use std::error::Error;

/// Representation of a password that has been securely hashed
#[derive(Debug, PartialEq, Clone, FromSql)]
pub struct Password(String);

/// Enumeration of errors that can occur when hashing a password
#[derive(Debug, PartialEq)]
pub struct PasswordHashError {}

impl std::fmt::Display for PasswordHashError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error hashing password")
    }
}

impl std::error::Error for PasswordHashError {}

impl From<BcryptError> for PasswordHashError {
    fn from(_: BcryptError) -> Self {
        PasswordHashError {}
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
            .is_equal_to(PasswordHashError {});
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

        let result = database.client().query("SELECT 'password_hash'", &[]);

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
        let result = database.client().query("SELECT 1", &[]);
        let rows = result.unwrap();
        assert_that(&rows.len()).is_equal_to(1);

        let row = rows.get(0).unwrap();
        assert_that(&row.len()).is_equal_to(1);

        let output_value: Result<Password, Error> = row.try_get(0);
        assert_that(&output_value).is_err();
        let err_msg = format!("{}", output_value.unwrap_err());
        assert_that(&err_msg)
                        .is_equal_to("error deserializing column 0: cannot convert between the Rust type `universe::users::model::password::Password` and the Postgres type `int4`".to_owned());
    }
}
