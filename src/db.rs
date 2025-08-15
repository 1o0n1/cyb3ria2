use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn connect_db(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Не удалось подключиться к базе данных")
}
