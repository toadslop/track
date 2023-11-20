use crate::domain::user::{CreateUserDto, User};
use chrono::Utc;
use secrecy::ExposeSecret;
use sqlx::{Pool, Postgres};
use std::ops::{Deref, DerefMut};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Database(Pool<Postgres>);

impl Database {
    pub async fn insert_user(&self, user_dto: &CreateUserDto) -> Result<User, sqlx::Error> {
        sqlx::query(
            r#"
            INSERT INTO user_ (id, email, name, password, created_at)
            VALUES($1, $2, $3, $4, $5)
        "#,
        )
        .bind(Uuid::new_v4())
        .bind(&user_dto.email)
        .bind(&user_dto.name)
        .bind(user_dto.password.expose_secret())
        .bind(Utc::now())
        .execute(&self.0)
        .await?;

        let user = sqlx::query_as::<_, User>("SELECT * FROM user_ WHERE email = $1")
            .bind(&user_dto.email)
            .fetch_one(&self.0)
            .await?;

        Ok(user)
    }
}

impl From<Pool<Postgres>> for Database {
    fn from(value: Pool<Postgres>) -> Self {
        Self(value)
    }
}

impl Deref for Database {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Database {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
