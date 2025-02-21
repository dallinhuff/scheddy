use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// An account-holding user of the application.
///
/// As a user represents a domain entity, its equality, ordering, and hash implementations are
/// based on its ID rather than the value of all of its fields.
#[derive(Debug, Clone)]
pub struct User {
    id: UserId,
    email: EmailAddress,
    password: Arc<str>,
    full_name: Arc<str>,
}

impl User {
    /// Creates a new user and assign them a unique [`UserId`].
    pub fn new(
        email: EmailAddress,
        password: impl Into<Arc<str>>,
        full_name: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            id: UserId::new(),
            email,
            password: password.into(),
            full_name: full_name.into(),
        }
    }

    /// Creates an instance of [`User`] from an existing [`UserId`] and fields.
    pub fn existing(
        id: UserId,
        email: EmailAddress,
        password: impl Into<Arc<str>>,
        full_name: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            id,
            email,
            password: password.into(),
            full_name: full_name.into(),
        }
    }

    /// The user's unique, immutable, and persistent identifier.
    #[must_use]
    pub fn id(&self) -> UserId {
        self.id
    }

    /// The user's email address used to log in and send communications to.
    #[must_use]
    pub fn email(&self) -> &EmailAddress {
        &self.email
    }

    /// Updates/changes the user's email address.
    pub fn set_email(&mut self, email: EmailAddress) -> &mut Self {
        self.email = email;
        self
    }

    /// The user's hashed & salted password.
    #[must_use]
    pub fn password(&self) -> &str {
        &self.password
    }

    /// Updates/changes the user's hashed & salted password.
    pub fn set_password(&mut self, password: impl Into<Arc<str>>) -> &mut Self {
        self.password = password.into();
        self
    }

    /// The user's full name given at registration.
    #[must_use]
    pub fn full_name(&self) -> &str {
        &self.full_name
    }

    /// Updates/changes the user's full name.
    pub fn set_full_name(&mut self, full_name: impl Into<Arc<str>>) -> &mut Self {
        self.full_name = full_name.into();
        self
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for User {}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("Invalid email address: {0}")]
    InvalidEmailAddress(Arc<str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(uuid::Uuid);

impl UserId {
    #[must_use]
    pub fn new() -> Self {
        Self(uuid::Uuid::now_v7())
    }

    #[must_use]
    pub const fn from_uuid(id: uuid::Uuid) -> Self {
        Self(id)
    }

    #[must_use]
    pub const fn inner(&self) -> uuid::Uuid {
        self.0
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<uuid::Uuid> for UserId {
    fn from(id: uuid::Uuid) -> Self {
        Self::from_uuid(id)
    }
}

/// A valid email address for a [`User`].
#[derive(Debug, Clone)]
pub struct EmailAddress(Arc<str>);

impl EmailAddress {
    /// Creates and validates an [`EmailAddress`] from a string slice.
    ///
    /// # Errors
    /// Returns a [`UserError`] if the provided email address is invalid.
    ///
    /// ```
    /// # use domain::user::EmailAddress;
    /// let email = EmailAddress::new("badEmail");
    /// assert!(email.is_err())
    /// ```
    pub fn new(email: impl Into<Arc<str>>) -> Result<Self, UserError> {
        let email = email.into();
        if Self::validate(&email) {
            Ok(Self(email))
        } else {
            Err(UserError::InvalidEmailAddress(email))
        }
    }

    /// Creates an [`EmailAddress`] from a string slice without validating it.
    /// Should only be used when the email's validation invariants are known to be upheld.
    #[cfg(feature = "unchecked-constructors")]
    pub fn new_unchecked(email: impl Into<Arc<str>>) -> Self {
        Self(email.into())
    }

    /// Get the raw/backing `&str` for the email address.
    #[must_use]
    pub fn inner(&self) -> &str {
        &self.0
    }

    /// Verify/validate that a given `&str` is a valid email address.
    fn validate(email: &str) -> bool {
        if email.is_empty() {
            return false;
        }

        let [local, domain] = email.split('@').collect::<Vec<_>>()[..] else {
            return false;
        };

        if local.is_empty() || domain.is_empty() {
            return false;
        }

        let [.., _, tld] = domain.split('.').collect::<Vec<_>>()[..] else {
            return false;
        };

        if tld.len() < 2 {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_address_validation() {
        for (given, expected) in [
            ("example@example.com", true),
            ("example.example@example.example.com", true),
            ("example@example.c", false),
            ("@example.com", false),
            ("example@example", false),
            ("example@example.", false),
            ("example.com", false),
            ("", false),
        ] {
            let actual = EmailAddress::validate(given);

            assert_eq!(
                expected, actual,
                "expected `{given}` to validate to `{expected}`"
            );
        }
    }
}
