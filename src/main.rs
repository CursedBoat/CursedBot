mod common;
mod events;
mod commands;

use common::{current_time, database::{get_pool, setup_pool}, read_json::config, structs::Config};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;
use std::sync::{Arc, RwLock};

// load global configuration
lazy_static!(
    static ref CONFIG: RwLock<Config> = RwLock::new(config());
);

// store current time for uptime command
lazy_static!(
    static ref STARTTIME: RwLock<u128> = RwLock::new(current_time());
);

// initialize database pool
static POOL: OnceCell<Arc<SqlitePool>> = OnceCell::new();



#[tokio::main]
async fn main() {
    // initialize dotenv and tracing
    tracing_subscriber::fmt::init();
    dotenv::dotenv().ok();

    // initialize static variables
    lazy_static::initialize(&STARTTIME);
    lazy_static::initialize(&CONFIG);

    let config = CONFIG.read().unwrap().clone();

    // set up database for use
    setup_pool().await;
    let database = get_pool();

    // run sql migrations
    sqlx::migrate!("./migrations").run(&*database).await.expect("Couldn't run db migrations.");

    // initialize framework and client
    let framework = common::framework::cursedbot_framework(config.clone());
    let mut client = common::client::cursedbot_client(framework, &config.client_config.token).await.unwrap();

    // start the client
    client.start().await.unwrap();
}
