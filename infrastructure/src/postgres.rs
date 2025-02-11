use sqlx::PgPool;

mod offering;
mod user;
mod vendor;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    /// Creates a new instance of [Postgres] from a connection string, which is used to
    /// initialize and reserve a connection pool to a database.
    ///
    /// # Errors
    /// - A [`sqlx::Error`] indicates that the connection pool failed to instantiate & connect
    ///   to the DB using the given connection string.
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;

        Ok(Self { pool })
    }
}
