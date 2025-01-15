use sqlx::{postgres::PgPoolOptions, Row};

async fn check_database_connection(database_url: &str) -> Result<(), sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    sqlx::query("SELECT 1").execute(&pool).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@db:5432/tests".to_string());

    if std::env::args().any(|arg| arg == "--health-check") {
        return check_database_connection(&database_url).await;
    }

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            nom VARCHAR NOT NULL,
            prenom VARCHAR NOT NULL
        )
        "#,
    )
    .execute(&pool)
    .await?;

    let count: i64 = sqlx::query("SELECT COUNT(*) FROM users")
        .fetch_one(&pool)
        .await?
        .get(0);

    if count == 0 {
        sqlx::query(
            r#"
            INSERT INTO users (nom, prenom) 
            VALUES ($1, $2)
            "#,
        )
        .bind("tavan")
        .bind("david")
        .execute(&pool)
        .await?;
        println!("Added default user: david tavan");
    }

    let users = sqlx::query("SELECT nom, prenom FROM users")
        .fetch_all(&pool)
        .await?;

    println!("Current users:");
    for user in users {
        println!(
            "{} {}",
            user.get::<String, _>("prenom"),
            user.get::<String, _>("nom")
        );
    }

    Ok(())
}
