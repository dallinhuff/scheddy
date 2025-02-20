use crate::postgres::Postgres;
use anyhow::Context;
use application::user::ports::repository::{Error, UserRepository};
use domain::user::{EmailAddress, User, UserId};
use uuid::Uuid;

impl UserRepository for Postgres {
    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, Error> {
        let result = sqlx::query_as!(
            AppUserDto,
            "SELECT app_user_id, email, full_name FROM app_user WHERE app_user_id = $1",
            id.inner(),
        )
        .fetch_optional(&self.pool)
        .await
        .context("failed to fetch user by ID")?;

        Ok(result.map(Into::into))
    }

    async fn get_by_email(&self, email: EmailAddress) -> Result<Option<User>, Error> {
        let result = sqlx::query_as!(
            AppUserDto,
            "SELECT app_user_id, email, full_name FROM app_user WHERE email = $1",
            email.inner(),
        )
        .fetch_optional(&self.pool)
        .await
        .context("failed to fetch user by email")?;

        Ok(result.map(Into::into))
    }

    async fn create(&self, user: User, password: String) -> Result<User, Error> {
        let dto = AppUserDto::from(user);

        sqlx::query!(
            "INSERT INTO app_user (app_user_id, email, password, full_name) VALUES ($1, $2, $3, $4)",
            dto.app_user_id,
            dto.email,
            password,
            dto.full_name,
        )
        .execute(&self.pool)
        .await
        .context("failed to create user")?;

        Ok(dto.into())
    }

    async fn delete(&self, user: User) -> Result<(), Error> {
        let dto = AppUserDto::from(user);

        sqlx::query!(
            "DELETE FROM app_user WHERE app_user_id = $1",
            dto.app_user_id
        )
        .execute(&self.pool)
        .await
        .context("failed to delete user")?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct AppUserDto {
    app_user_id: Uuid,
    email: String,
    full_name: String,
}

impl From<AppUserDto> for User {
    fn from(dto: AppUserDto) -> Self {
        User::existing(
            dto.app_user_id.into(),
            EmailAddress::new(dto.email).unwrap(),
            dto.full_name,
        )
    }
}

impl From<User> for AppUserDto {
    fn from(model: User) -> Self {
        AppUserDto {
            app_user_id: model.id().inner(),
            email: model.email().inner().into(),
            full_name: model.full_name().into(),
        }
    }
}
