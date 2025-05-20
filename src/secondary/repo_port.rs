use async_trait::async_trait;
use mockall::automock;

use crate::domain::model::User;

#[automock]
#[async_trait]
pub trait UserRepo {
    async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error>;
    async fn insert_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<User, sqlx::Error>;
}
