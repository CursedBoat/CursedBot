use std::sync::Arc;
use sqlx::{query, sqlite::SqlitePoolOptions, SqlitePool};

use crate::POOL;

pub async fn init_pool() -> Arc<SqlitePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::new()
                .filename("database.sqlite")
                .create_if_missing(true),
        ).await.expect("Couldn't connect to DB");
    
    // create config table if it does not exist
    let create_table_sql = format!(
        "CREATE TABLE IF NOT EXISTS config (
            guild_id INTEGER NOT NULL,
            qotd_channel TEXT,
            sotd_channel TEXT,
            qotd_suggestions_channel TEXT,
            sotd_suggestions_channel TEXT,
            xotd_enable	INTEGER NOT NULL,
            roleconfig_userperm	INTEGER,
            PRIMARY KEY(guild_id AUTOINCREMENT)
        );"
    );

    query(&create_table_sql)
        .execute(&pool)
        .await
        .unwrap();
    
    Arc::new(pool)
}

pub async fn setup_pool() {
    let pool = init_pool().await;
    POOL.set(pool).expect("failed to set pool");
}

pub fn get_pool() -> Arc<SqlitePool> {
    POOL.get().expect("Pool not initialized").clone()
}