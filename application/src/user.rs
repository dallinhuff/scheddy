pub mod service {
    use crate::user::ports::UserRepository;

    /// A service responsible for managing the registration,
    /// login, and account management of [User]s.
    ///
    /// [User]: domain::user::User
    pub trait UserService {}

    /// Errors that may occur when interacting with a [UserService].
    #[derive(Debug, Clone, PartialEq)]
    pub enum Error {}

    /// An implementation of [UserService] that uses a [UserRepository]
    /// to access & persist user data.
    #[derive(Debug, Clone)]
    pub struct UserServiceLive<R: UserRepository> {
        repo: R,
    }

    impl<R: UserRepository> UserServiceLive<R> {
        pub fn new(repo: R) -> UserServiceLive<R> {
            Self { repo }
        }
    }

    impl<R: UserRepository> UserService for UserServiceLive<R> {}
}

/// Ports that outside adapters may implement for use with [service].
pub mod ports {
    use domain::user::{EmailAddress, User, UserId, Username};
    use std::future::Future;

    /// A persistent data store for [User] models.
    ///
    /// [User]: domain::user::User
    pub trait UserRepository {
        /// Fetch a [User] by their ID.
        fn get_by_id(&self, id: UserId) -> impl Future<Output = ResultOpt<User>>;

        /// Fetch a [User] by their unique username.
        fn get_by_username(&self, username: Username) -> impl Future<Output = ResultOpt<User>>;

        /// Fetch a [User] by their unique email address.
        fn get_by_email(&self, email: EmailAddress) -> impl Future<Output = ResultOpt<User>>;

        /// Create a new [User]
        fn create(&self, user: User) -> impl Future<Output = Result<User>>;

        /// Delete a [User]
        fn delete(&self, user: User) -> impl Future<Output = Result<()>>;
    }

    type Result<T> = std::result::Result<T, RepositoryError>;
    type ResultOpt<T> = Result<Option<T>>;

    #[derive(Debug, Clone, PartialEq)]
    pub enum RepositoryError {
        Unknown,
    }
}
