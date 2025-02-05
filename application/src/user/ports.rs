pub mod repository {
    use domain::user::{EmailAddress, User, UserId, Username};

    #[trait_variant::make(Send)]
    pub trait UserRepository: Sync {
        async fn get_by_id(&self, id: UserId) -> Result<Option<User>, Error>;
        async fn get_by_username(&self, username: Username) -> Result<Option<User>, Error>;
        async fn get_by_email(&self, email: EmailAddress) -> Result<Option<User>, Error>;
        async fn create(&self, user: User) -> Result<User, Error>;
        async fn delete(&self, user: User) -> Result<(), Error>;
    }

    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        Unknown(#[from] anyhow::Error),
    }
}
