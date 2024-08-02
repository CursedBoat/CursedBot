mod common;
mod events;
mod commands;

use axum::{routing::get, Router};
use common::{current_time, database::{get_pool, setup_pool}, read_json::config, structs::Config};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sqlx::SqlitePool;
use tokio::net::TcpListener;
use tracing::info;
use std::{net::SocketAddr, sync::{Arc, RwLock}};

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

    // read config
    let config = CONFIG.read().unwrap().clone();
    match config.bot_config.axum_server {
        true => {
            // start both the tasks

            // tokio task for running the serenity bot
            let serenity_future = tokio::spawn(async {
                serenity_server().await
            });

            // tokio task for running a minimal server you can ping to keep the bot alive
            // in free server hosts.
            let axum_future = tokio::spawn(async {
                axum_server().await
            });

            tokio::select! {
                _ = serenity_future => { println!("Serenity bot has terminated."); }
                _ = axum_future => { println!("Axum server has terminated."); }
            }
        },
        false => {
            // start only the bot
            serenity_server().await
        }
    };
}

// function for starting the serenity client
async fn serenity_server() {
    // read config
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

// function for starting the axum server
async fn axum_server() {
    // init variables
    let listener: TcpListener;
    async fn server_message() -> &'static str {
        "The bot is running!"
    }

    // axum router
    let app = Router::new().route("/", get(server_message));

    // bind address
    let socketaddress = "127.0.0.1:3000";
    match TcpListener::bind(socketaddress).await {
        Ok(listener_ok) => { 
            listener = listener_ok; 
            info!("Axum server running on {}", socketaddress);
        }
        Err(e) => {
            info!("Error binding to port: {}", e);
            std::process::exit(1);
        }
    }

    // serve the axum server
    let server = axum::serve(
        listener, 
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await;

    // handle errors
    match server{
        Ok(_) => {},
        Err(e) => { info!("Error occured! Err: {}", e) }
    }
}