use std::env;

use rust_hexa_postgres::{
    application::user_service::UserService,
    primary::{cli_adapter::CliAdapter, cli_port::CliPort, http_port::HttpUserPort},
    secondary::psql_repo::PsqlRepo,
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL")?;

    let pool_result = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let repo = PsqlRepo::new(pool_result);
    let service = UserService::new(repo);

    // * CLI adapter
    let cli_adapter = CliAdapter::new(service);
    cli_adapter.list_users().await?;

    // * HTTP adapter
    // let http_adapter = rust_hexa_postgres::primary::http_adapter::HttpAdapter::new(service);
    // let addr = "127.0.0.1:3000";

    // let _ = http_adapter.run(addr).await;

    Ok(())
}
