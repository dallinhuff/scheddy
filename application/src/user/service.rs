use crate::user::ports::repository::UserRepository;
use domain::user::{EmailAddress, User};

/// Instances of `UserService` are able to interact with & manage [User][domain::user::User]s.
#[trait_variant::make(Send)]
pub trait UserService {
    /// Finds a user by email and password.
    /// Both the email and password must be an exact match to return Ok(Some(User)).
    ///
    /// # Errors
    /// - [Error::Repository] if an unexpected error occurs while fetching a user.
    async fn get_by_email_and_password(
        &self,
        email: EmailAddress,
        password: &str,
    ) -> Result<Option<User>, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Domain(#[from] domain::user::UserError),

    #[error(transparent)]
    Repository(#[from] crate::user::ports::repository::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

/// Instances of `HashingService` are able to hash and verify passwords.
pub trait HashingService: Send + Sync {
    /// Creates a hashed password from a given password key.
    #[must_use]
    fn hash(&self, password: &str) -> String;

    /// Verify a password against a hashed password to check if they are equal.
    #[must_use]
    fn verify(&self, password: &str, hashed_password: &str) -> bool;
}

/// A live implementation of [`UserService`].
#[derive(Debug, Clone)]
pub struct UserServiceLive<R: UserRepository, H: HashingService> {
    repo: R,
    hasher: H,
}

impl<R: UserRepository, H: HashingService> UserServiceLive<R, H> {
    /// Creates a new instance of [`UserServiceLive`] for use where a [`UserService`] is expected.
    pub fn new(repo: R, hasher: H) -> UserServiceLive<R, H> {
        Self { repo, hasher }
    }
}

impl<R: UserRepository, H: HashingService> UserService for UserServiceLive<R, H> {
    async fn get_by_email_and_password(
        &self,
        email: EmailAddress,
        password: &str,
    ) -> Result<Option<User>, Error> {
        let user = self
            .repo
            .get_by_email(email)
            .await?
            .filter(|user| self.hasher.verify(password, user.password()));

        Ok(user)
    }
}

pub struct HashingServiceLive;

impl HashingService for HashingServiceLive {
    fn hash(&self, password: &str) -> String {
        password_auth::generate_hash(password)
    }

    fn verify(&self, password: &str, hashed_password: &str) -> bool {
        password_auth::verify_password(password, hashed_password).is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hashing_service_works() {
        let service = HashingServiceLive;

        let password = "a_badPassword1@";
        let hash = service.hash(password);
        let other_password = "b_badPassword2";
        let other_hash = service.hash(other_password);

        assert_ne!(password, hash, "Hash value should not be the same as key");
        assert_ne!(hash, other_hash, "Different keys should not collide");
        assert!(
            service.verify(password, &hash),
            "Verify should succeed when given correct key-value"
        );
        assert!(
            !service.verify(other_password, &hash),
            "Verify should fail when given incorrect key-value"
        );
    }
}
