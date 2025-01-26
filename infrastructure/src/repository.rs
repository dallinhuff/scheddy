use application::ports::WaiverRepository;
use domain::waiver::{Waiver, WaiverId};

pub struct PgWaiverRepository {
    pool: sqlx::PgPool,
}

impl PgWaiverRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

impl WaiverRepository for PgWaiverRepository {
    async fn get_by_id(&self, id: WaiverId) -> Result<Option<Waiver>, String> {
        sqlx::query_as!(Waiver, "select id, content from waiver where id = $1", id.0)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }

    async fn save(&self, waiver: Waiver) -> Result<Waiver, String> {
        sqlx::query!(
            "insert into waiver (id, content) values ($1, $2)",
            waiver.id.0,
            waiver.content
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())
        .and_then(|r| {
            if r.rows_affected() != 1 {
                Err("".to_string())
            } else {
                Ok(waiver)
            }
        })
    }

    async fn delete(&self, waiver: Waiver) -> Result<(), String> {
        let WaiverId(inner) = waiver.id;

        sqlx::query!("delete from waiver where id = $1", inner)
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())
            .and_then(|r| {
                if r.rows_affected() != 1 {
                    Err("".to_string())
                } else {
                    Ok(())
                }
            })
    }
}
