use crate::user::ports::repository::UserRepository;
use domain::user::{User, UserId};

#[trait_variant::make(Send)]
pub trait UserService {
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Domain(#[from] domain::user::UserError),

    #[error(transparent)]
    Repository(#[from] super::ports::repository::Error),

    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

#[derive(Debug, Clone)]
pub struct UserServiceLive<R: UserRepository> {
    repo: R,
}

impl<R: UserRepository> UserServiceLive<R> {
    pub fn new(repo: R) -> UserServiceLive<R> {
        Self { repo }
    }
}

impl<R: UserRepository> UserService for UserServiceLive<R> {
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, Error> {
        let user = self.repo.get_by_id(id).await?;

        Ok(user)
    }
}
