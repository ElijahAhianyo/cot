//! Form Field Types for Cot
//!
//! This module provides a collection of form field types and utilities for validating,
//! parsing, and converting user input within Cot. It includes general-purpose newtype wrappers
//! and associated trait implementations to ensure consistent and safe processing of form data.

use std::str::FromStr;

#[cfg(feature = "db")]
use crate::db::{ColumnType, DatabaseField, DbValue, FromDbValue, SqlxValueRef, ToDbValue};
use cot::db::impl_mysql::MySqlValueRef;
use cot::db::impl_postgres::PostgresValueRef;
use cot::db::impl_sqlite::SqliteValueRef;
use email_address::EmailAddress;

// Maximum email length as specified in the RFC 5321
const MAX_EMAIL_LENGTH: u32 = 254;

/// A validated email address.
///
/// This is a newtype wrapper around [`EmailAddress`](email_address::EmailAddress) that provides
/// validation and integration with Cot's database system. It ensures email addresses
/// comply with RFC 5321/5322 standards.
///
/// # Examples
///
/// ```
/// use cot::form::types::Email;
/// use std::str::FromStr;
///
/// // Parse from a string
/// let email = Email::from_str("user@example.com").unwrap();
///
/// // Convert using TryFrom
/// let email = Email::try_from("user@example.com").unwrap();
/// ```
#[derive(Clone, Debug)]
pub struct Email(EmailAddress);

impl Email {
    /// Creates a new `Email` from a string, validating that it's a proper email address.
    ///
    /// # Errors
    ///
    /// Returns an error if the email address is invalid according to RFC standards.
    ///
    /// # Examples
    ///
    /// ```
    /// use cot::form::types::Email;
    ///
    /// let email = Email::new("user@example.com").unwrap();
    /// assert!(Email::new("invalid").is_err());
    /// ```
    pub fn new<S: AsRef<str>>(email: S) -> Result<Email, email_address::Error> {
        EmailAddress::from_str(email.as_ref()).map(Self)
    }

    /// Returns the email address as a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cot::form::types::Email;
    ///
    /// let email = Email::from_str("user@example.com").unwrap();
    /// assert_eq!(email.as_str(), "user@example.com");
    /// ```
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns a reference to the underlying `EmailAddress`.
    ///
    /// This is useful when you need to access functionality provided by the
    /// `email_address` crate directly.
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use cot::form::types::Email;
    ///
    /// let email = Email::from_str("user@example.com").unwrap();
    /// let domain = email.as_inner().domain();
    /// assert_eq!(domain, "example.com");
    /// ```
    #[must_use] pub fn as_inner(&self) -> &EmailAddress {
        &self.0
    }
}

/// Implements string parsing for `Email`.
///
/// # Examples
///
/// ```
/// use std::str::FromStr;
/// use cot::form::types::Email;
///
/// let email = Email::from_str("user@example.com").unwrap();
/// ```
impl FromStr for Email {
    type Err = email_address::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Email::new(s)
    }
}

/// Implements conversion from string references to `Email`.
///
/// # Examples
///
/// ```
/// use cot::form::types::Email;
///
/// let email = Email::try_from("user@example.com").unwrap();
/// ```
impl TryFrom<&str> for Email {
    type Error = email_address::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

/// Implements conversion from `String` to `Email`.
///
/// # Examples
///
/// ```
/// use cot::form::types::Email;
///
/// let email = Email::try_from(String::from("user@example.com")).unwrap();
/// ```
#[cfg(feature = "db")]
impl TryFrom<String> for Email {
    type Error = email_address::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Email::new(value)
    }
}

/// Implements database value conversion for `Email`.
///
/// This allows `Email` to be stored in the database as a text value.
impl ToDbValue for Email {
    fn to_db_value(&self) -> DbValue {
        self.0.clone().to_string().into()
    }
}

/// Implements database value conversion for retrieving `Email` from the database.
///
/// This allows `Email` to be retrieved from the database and properly converted and validated.
#[cfg(feature = "db")]
impl FromDbValue for Email {
    #[cfg(feature = "sqlite")]
    fn from_sqlite(value: SqliteValueRef<'_>) -> cot::db::Result<Self>
    where
        Self: Sized,
    {
        Email::new(value.get::<String>()?).map_err(cot::db::DatabaseError::value_decode)
    }

    #[cfg(feature = "postgres")]
    fn from_postgres(value: PostgresValueRef<'_>) -> cot::db::Result<Self>
    where
        Self: Sized,
    {
        Email::new(value.get::<String>()?).map_err(cot::db::DatabaseError::value_decode)
    }

    #[cfg(feature = "mysql")]
    fn from_mysql(value: MySqlValueRef<'_>) -> cot::db::Result<Self>
    where
        Self: Sized,
    {
        Email::new(value.get::<String>()?).map_err(cot::db::DatabaseError::value_decode)
    }
}

/// Defines the database field type for `Email`.
///
/// Emails are stored as strings with a maximum length of 254 characters,
/// as specified in RFC 5321.
#[cfg(feature = "db")]
impl DatabaseField for Email {
    const TYPE: ColumnType = ColumnType::String(MAX_EMAIL_LENGTH);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    

    #[test]
    fn test_valid_email_creation() {
        let email = Email::new("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
        assert_eq!(email.as_inner().domain(), "example.com");
    }

    #[test]
    fn test_invalid_email_creation() {
        let result = Email::new("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str_trait() {
        let email: Email = "user@example.com".parse().unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[test]
    fn test_try_from_trait() {
        let email = Email::try_from("user@example.com").unwrap();
        assert_eq!(email.as_str(), "user@example.com");
    }

    #[cfg(feature = "db")]
    mod db_tests {
        use super::*;
        use crate::db::ToDbValue;

        #[test]
        fn test_to_db_value() {
            let email = Email::new("user@example.com").unwrap();
            let db_value = email.to_db_value();

            let email_str = email.as_str();
            let db_value_str = format!("{db_value:?}");
            assert!(db_value_str.contains(email_str));
        }
    }
}
