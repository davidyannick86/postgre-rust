use std::env;

use ports::primary::cli::CliPort;
use sqlx::postgres::PgPoolOptions;

mod adapters;
mod application;
mod domain;
mod ports;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match dotenvy::dotenv() {
        Ok(_) => println!("Loaded .env file"),
        Err(e) => println!("Error loading .env file: {}", e),
    }

    let database_url = match env::var("DATABASE_URL") {
        Ok(url) => url,
        Err(e) => {
            eprintln!("Error getting DATABASE_URL: {}", e);
            return Err(Box::<dyn std::error::Error>::from(e));
        }
    };

    let pool_result = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let repo = adapters::secondary::psql_repo::PsqlRepo::new(pool_result);
    let service = application::user_service::UserService::new(repo);

    let cli_adapter = adapters::primary::cli::CliAdapter::new(service);

    let _user = cli_adapter
        .add_user(
            "Georgen+kl44".into(),
            "djkjke@ldsa.jungledd".into(),
            "Password".into(),
            true,
        )
        .await;

    cli_adapter.list_users().await?;

    // let http_adapter = adapters::primary::http::HttpAdapter::new(service);
    // let addr = "127.0.0.1:3000";

    // let _ = http_adapter.run(addr).await;

    Ok(())
}
