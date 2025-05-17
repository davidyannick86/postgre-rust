use async_trait::async_trait;
use sqlx::query_as;

use crate::domain::model::User;
use crate::ports::secondary::repo::UserRepo;

pub struct PsqlRepo {
    repo: sqlx::PgPool,
}

impl PsqlRepo {
    pub fn new(repo: sqlx::PgPool) -> Self {
        PsqlRepo { repo }
    }
}

#[async_trait]
impl UserRepo for PsqlRepo {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let users = query_as!(
            User,
            r#"
            SELECT id, name, email, password, is_active
            FROM users
            "#
        )
        .fetch_all(&self.repo)
        .await?;

        Ok(users)
    }

    async fn insert_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<User, sqlx::Error> {
        let user = query_as!(
            User,
            r#"
            INSERT INTO users (name, email, password, is_active)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, password, is_active
            "#,
            name,
            email,
            password,
            is_active
        )
        .fetch_one(&self.repo)
        .await?;

        Ok(user)
    }
}
