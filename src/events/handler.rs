use poise::serenity_prelude as serenity;
use sqlx::query;
use tracing::info;
use crate::common::{database::get_pool, framework::{Data, Error}};

pub async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            tracing::info!("Logged in as {}", data_about_bot.user.name);
        },
        // create database entry when user joins server
        serenity::FullEvent::GuildCreate { guild, is_new } => {
            match is_new.unwrap() {
                true => {
                    let create_table_sql = format!(
                        "CREATE TABLE IF NOT EXISTS '{}' (
                            username TEXT,
                            role_id TEXT
                        );", guild.id
                    );

                    let insert_default_conf_sql = format!(
                        "INSERT INTO config (guild_id, xotd_enable, roleconfig_userperm) VALUES ($1, $2, $3)"
                    ); 

                    let database = get_pool();

                    // create database
                    query(&create_table_sql)
                        .execute(&*database)
                        .await?;

                    // insert default config
                    query(&insert_default_conf_sql)
                        .bind(guild.id.to_string())
                        .bind(0)
                        .bind(0)
                        .execute(&*database)
                        .await?;

                    // log events
                    info!("Joined guild {} id({}), Successfully added DB entry.", guild.name, guild.id);
                },
                false => {}
            }
        },
        // delete database entry when bot leaves server
        serenity::FullEvent::GuildDelete { incomplete, full: _ } => {
            let database = get_pool();

            let delete_table_sql = format!(
                "DROP TABLE IF EXISTS '{}'", incomplete.id
            );

            query(&delete_table_sql)
                    .execute(&*database)
                    .await?;
            
            info!("Left guild {}, Successfully dropped DB entry.", incomplete.id);
        }
        _ => {}
    }
    Ok(())
}