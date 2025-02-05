use crate::postgres::Postgres;
use application::user::ports::repository::{Error, UserRepository};
use domain::user::{EmailAddress, Password, User, UserId, Username};

impl UserRepository for Postgres {
    async fn get_by_id(&self, UserId(id): UserId) -> Result<Option<User>, Error> {
        let result = sqlx::query!(
            "select id, username, password, email from app_user where id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(|record| User {
            id: UserId(record.id),
            username: Username(record.username),
            password: Password(record.password),
            email: EmailAddress(record.email),
        }))
    }

    async fn get_by_username(&self, username: Username) -> Result<Option<User>, Error> {
        let result = sqlx::query!(
            "select id, username, password, email from app_user where username = $1",
            username.0,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(|record| User {
            id: UserId(record.id),
            username: Username(record.username),
            password: Password(record.password),
            email: EmailAddress(record.email),
        }))
    }

    async fn get_by_email(&self, email: EmailAddress) -> Result<Option<User>, Error> {
        let result = sqlx::query!(
            "select id, username, password, email from app_user where email = $1",
            email.0,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(|record| User {
            id: UserId(record.id),
            username: Username(record.username),
            password: Password(record.password),
            email: EmailAddress(record.email),
        }))
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        sqlx::query!(
            "insert into app_user (id, username, password, email) values ($1, $2, $3, $4)",
            user.id.0,
            user.username.0,
            user.password.0,
            user.email.0,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))
        .map(|_| user)
    }

    async fn delete(&self, user: User) -> Result<(), Error> {
        sqlx::query!("delete from app_user where id = $1", user.id.0)
            .execute(&self.pool)
            .await
            .map_err(|e| Error::Unknown(e.into()))?;

        Ok(())
    }
}
