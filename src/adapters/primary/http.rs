use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Json, Router,
    extract::State,
    http::StatusCode,
    routing::{get, post},
};
use serde::Deserialize;
use tokio::net::TcpListener;

use crate::{
    application::user_service::UserService,
    domain::model::User,
    ports::{primary::http::HttpUserPort, secondary::repo::UserRepo},
};

#[allow(dead_code)]
#[derive(Deserialize)]
struct AddUserPayload {
    name: String,
    email: String,
    password: String,
    is_active: bool,
}

pub struct HttpAdapter<R: UserRepo + Send + Sync + 'static> {
    user_service: Arc<UserService<R>>,
}

#[allow(dead_code)]
impl<R: UserRepo + Send + Sync + 'static> HttpAdapter<R> {
    pub fn new(user_service: UserService<R>) -> Self {
        HttpAdapter {
            user_service: Arc::new(user_service),
        }
    }

    pub async fn run(self, addr: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Créer des wrappers non-génériques pour nos fonctions
        // qui capturent le type R spécifique
        let http_adapter = Arc::new(self);

        let list_users =
            move |state: State<Arc<HttpAdapter<R>>>| async move { list_users_handler(state).await };

        let add_user = move |state: State<Arc<HttpAdapter<R>>>, payload: Json<AddUserPayload>| async move {
            add_user_handler(state, payload).await
        };

        let app = Router::new()
            .route("/users", get(list_users))
            .route("/users", post(add_user))
            // Share the HttpAdapter with Axum handlers
            .with_state(http_adapter.clone());

        let listener = TcpListener::bind(addr).await?;
        println!("HTTP server listening on {}", addr);
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[async_trait]
impl<R: UserRepo + Send + Sync + 'static> HttpUserPort for HttpAdapter<R> {
    async fn list_users(&self) -> Result<Vec<User>, String> {
        match self.user_service.get_all_users().await {
            Ok(users) => Ok(users),
            Err(e) => Err(e.to_string()),
        }
    }

    async fn add_user(
        &self,
        name: String,
        email: String,
        password: String,
        is_active: bool,
    ) -> Result<User, String> {
        match self
            .user_service
            .insert_user(name, email, password, is_active)
            .await
        {
            Ok(user) => Ok(user),
            Err(e) => Err(e.to_string()),
        }
    }
}

async fn list_users_handler(
    State(http_adapter): State<Arc<HttpAdapter<impl UserRepo + Send + Sync>>>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match http_adapter.list_users().await {
        Ok(users) => Ok(Json(users)),
        Err(e) => {
            eprintln!("HTTP Handler Error - Failed to retrieve users: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to retrieve users".to_string(),
            ))
        }
    }
}

async fn add_user_handler(
    State(http_adapter): State<Arc<HttpAdapter<impl UserRepo + Send + Sync>>>,
    Json(payload): Json<AddUserPayload>,
) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
    match http_adapter
        .add_user(
            payload.name,
            payload.email,
            payload.password,
            payload.is_active,
        )
        .await
    {
        Ok(user) => Ok((StatusCode::CREATED, Json(user))),
        Err(e) => {
            eprintln!("HTTP Handler Error - Failed to add user: {:?}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to add user".to_string(),
            ))
        }
    }
}
