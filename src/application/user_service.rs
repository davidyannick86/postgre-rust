use crate::domain::model::User;
use crate::secondary::repo_port::UserRepo;

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::model::User;
    use crate::secondary::repo_port::MockUserRepo;
    use mockall::predicate::*;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_get_all_users() {
        let mut mock_repo = MockUserRepo::new();
        let expected_users = vec![User {
            id: Uuid::new_v4(), // Changed to Uuid::new_v4()
            name: "Test User".to_string(),
            email: "test@example.com".to_string(),
            password: "password".to_string(),
            is_active: true,
        }];
        let expected_users_clone = expected_users.clone();

        mock_repo
            .expect_get_all_users()
            .times(1)
            .returning(move || Ok(expected_users_clone.clone()));

        let user_service = UserService::new(mock_repo);
        let result = user_service.get_all_users().await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_users);
    }

    #[tokio::test]
    async fn test_insert_user() {
        let mut mock_repo = MockUserRepo::new();
        let name = "New User".to_string();
        let email = "new@example.com".to_string();
        let password = "securepassword".to_string();
        let is_active = true;

        let expected_user = User {
            id: Uuid::new_v4(),
            name: name.clone(),
            email: email.clone(),
            password: password.clone(),
            is_active,
        };
        let expected_user_clone = expected_user.clone();

        mock_repo
            .expect_insert_user()
            .with(
                eq(name.clone()),
                eq(email.clone()),
                eq(password.clone()),
                eq(is_active),
            )
            .times(1)
            .returning(move |_, _, _, _| Ok(expected_user_clone.clone()));

        let user_service = UserService::new(mock_repo);
        let result = user_service
            .insert_user(name, email, password, is_active)
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), expected_user);
    }
}
