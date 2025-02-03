use sqlx::PgPool;

mod offering;
mod user;
mod vendor;

#[derive(Debug, Clone)]
pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;

        Ok(Self { pool })
    }
}
