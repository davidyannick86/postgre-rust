use crate::domain::model::User;
use async_trait::async_trait;

#[async_trait]
pub trait HttpUserPort {
    async fn list_users(&self) -> Result<Vec<User>, String>;
    async fn add_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<User, String>;
    async fn run(
        self,
        addr: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>;
}
