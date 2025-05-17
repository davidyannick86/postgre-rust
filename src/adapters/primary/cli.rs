use crate::{
    application::user_service::UserService,
    ports::{primary::cli::CliPort, secondary::repo::UserRepo},
};

pub struct CliAdapter<R: UserRepo + Send + Sync> {
    user_service: UserService<R>,
}

impl<R: UserRepo + Send + Sync> CliAdapter<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        CliAdapter { user_service }
    }
}

#[async_trait::async_trait]
impl<R: UserRepo + Send + Sync> CliPort for CliAdapter<R> {
    async fn list_users(&self) -> Result<(), String> {
        let users = self
            .user_service
            .get_all_users()
            .await
            .map_err(|e| e.to_string())?;

        for user in users {
            println!(
                "ID: {}, Name: {}, Email: {}, Password: {}, Is Active: {}",
                user.id, user.name, user.email, user.password, user.is_active
            );
        }
        Ok(())
    }

    async fn add_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<(), String> {
        self.user_service
            .insert_user(name, email, password, is_active)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
