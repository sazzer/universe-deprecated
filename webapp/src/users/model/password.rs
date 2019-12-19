use bytes::BytesMut;
use postgres::types::{accepts, to_sql_checked, FromSql, IsNull, ToSql, Type};
use serde::{Serialize, Serializer};
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub struct Password(String);

impl Password {
    pub fn new_from_hash<S>(hash: S) -> Password
    where
        S: Into<String>,
    {
        Password(hash.into())
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
        String::from_sql(t, raw).map(|u| Password(u))
    }
    accepts!(VARCHAR, TEXT);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::test::TestDatabase;
    use postgres::Error;
    use spectral::prelude::*;
    use speculate::speculate;

    speculate! {
        before {
            let _ = env_logger::try_init();
        }

        describe "serde" {
            describe "Serialize" {
                test "Serializing a password always returns null" {
                    let password = Password::new_from_hash("password_hash");

                    let serialized = serde_json::to_value(password);
                    assert_that(&serialized).is_ok().is_equal_to(serde_json::Value::Null);
                }
            }
        }

        describe "postgres" {
            before {
                let database = TestDatabase::new();
                let mut client = database.client();
            }

            describe "ToSql" {
                test "Using a valid Username in a query" {
                    let password = Password::new_from_hash("password_hash");
                    let result = client.query("SELECT $1", &[&password]);

                    let rows = result.unwrap();
                    assert_that(&rows.len()).is_equal_to(1);

                    let row = rows.get(0).unwrap();
                    assert_that(&row.len()).is_equal_to(1);

                    let output_value: &str = rows.get(0).unwrap().get(0);
                    assert_that(&output_value).is_equal_to("password_hash");
                }
            }

            describe "FromSql" {
                test "Fetching a valid Username from a query" {
                    let result = client.query("SELECT $1", &[&"password_hash"]);

                    let rows = result.unwrap();
                    assert_that(&rows.len()).is_equal_to(1);

                    let row = rows.get(0).unwrap();
                    assert_that(&row.len()).is_equal_to(1);

                    let output_value: Password = rows.get(0).unwrap().get(0);
                    assert_that(&output_value).is_equal_to(Password::new_from_hash("password_hash"));
                }

                test "Fetching a number from a query" {
                    let result = client.query("SELECT $1", &[&1]);

                    let err: Error = result.err().unwrap();
                    let err_msg = format!("{}", err);
                    assert_that(&err_msg)
                        .is_equal_to("error serializing parameter 0: cannot convert between the Rust type `i32` and the Postgres type `text`".to_owned());
                }
            }
        }
    }
}
