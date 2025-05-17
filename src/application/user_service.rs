use crate::domain::model::User;
use crate::ports::secondary::repo::UserRepo;

pub struct UserService<R: UserRepo> {
    user_repo: R,
}

impl<R: UserRepo> UserService<R> {
    pub fn new(user_repo: R) -> Self {
        UserService { user_repo }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        self.user_repo.get_all_users().await
    }

    pub async fn insert_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<User, sqlx::Error> {
        self.user_repo
            .insert_user(name, email, password, is_active)
            .await
    }
}
