use crate::user::ports::repository;
use crate::user::ports::repository::UserRepository;
use domain::user::{User, UserId};

#[trait_variant::make(Send)]
pub trait UserService {
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, Error>;
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
}

impl From<repository::Error> for Error {
    fn from(error: repository::Error) -> Self {
        Error::Unknown(error.into())
    }
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
        self.repo.get_by_id(id).await.map_err(Into::into)
    }
}
