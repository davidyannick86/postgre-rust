use async_trait::async_trait;

#[async_trait]
pub trait CliPort {
    async fn list_users(&self) -> Result<(), String>;
    async fn add_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<(), String>;
}
