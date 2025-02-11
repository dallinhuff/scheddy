use crate::postgres::Postgres;
use application::user::ports::repository::{Error, UserRepository};
use domain::user::{EmailAddress, Password, User, UserId, Username};
use uuid::Uuid;

impl UserRepository for Postgres {
    async fn get_by_id(&self, UserId(id): UserId) -> Result<Option<User>, Error> {
        let result = sqlx::query_as!(
            AppUserDto,
            "SELECT * FROM app_user WHERE app_user_id = $1",
            id,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(Into::into))
    }

    async fn get_by_username(&self, username: Username) -> Result<Option<User>, Error> {
        let result = sqlx::query_as!(
            AppUserDto,
            "SELECT * FROM app_user WHERE username = $1",
            username.0,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(Into::into))
    }

    async fn get_by_email(&self, email: EmailAddress) -> Result<Option<User>, Error> {
        let result = sqlx::query_as!(
            AppUserDto,
            "SELECT * FROM app_user WHERE email = $1",
            email.0,
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(result.map(Into::into))
    }

    async fn create(&self, user: User) -> Result<User, Error> {
        let dto = AppUserDto::from(user);

        sqlx::query!(
            "INSERT INTO app_user (app_user_id, username, password, email) VALUES ($1, $2, $3, $4)",
            dto.app_user_id,
            dto.username,
            dto.password,
            dto.email,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| Error::Unknown(e.into()))?;

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
        .map_err(|e| Error::Unknown(e.into()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct AppUserDto {
    app_user_id: Uuid,
    username: String,
    password: String,
    email: String,
}

impl From<AppUserDto> for User {
    fn from(dto: AppUserDto) -> Self {
        User {
            id: UserId(dto.app_user_id),
            username: Username(dto.username),
            password: Password(dto.password),
            email: EmailAddress(dto.email),
        }
    }
}

impl From<User> for AppUserDto {
    fn from(model: User) -> Self {
        AppUserDto {
            app_user_id: model.id.0,
            username: model.username.0,
            password: model.password.0,
            email: model.email.0,
        }
    }
}
