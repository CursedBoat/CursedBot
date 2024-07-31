use std::sync::Arc;
use poise::{serenity_prelude as serenity, CreateReply};
use sqlx::{Pool, Sqlite};
use crate::common::structs::GuildConfig;
use crate::common::framework::Context;

use super::structs::{Config, GuildConfigCheck};

// check if xotd is enabled in a server config
pub async fn check_xotd(
    ctx: Context<'_>, 
    guild_id: i64, database: Arc<Pool<Sqlite>>, 
    config: Config
) -> GuildConfigCheck {
    let enabled: bool;

    // sql query to grab guild config from database
    let guild_config = sqlx::query_as!(
        GuildConfig,
        "SELECT * FROM config WHERE guild_id = ?",
        guild_id
    )
    .fetch_one(&*database)
    .await.unwrap().clone();
    
    // send error message if the command is disabled
    if guild_config.xotd_enable.unwrap() == 0 {
        enabled = false;

        let embed = serenity::CreateEmbed::default()
        .title("Command disabled!")
        .description("Please edit the configuration to enable this command.")
        .color(config.color_config.pink);

        ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await.unwrap();

        return GuildConfigCheck {config: guild_config, enabled: enabled};
    };
    enabled = true;

    // return struct with guild config and enabled information
    return GuildConfigCheck {config: guild_config, enabled: enabled};
}

// check if role config is enabled in a server config
pub async fn check_role(
    ctx: Context<'_>, 
    guild_id: i64, database: Arc<Pool<Sqlite>>, 
    config: Config
) -> GuildConfigCheck {
    let enabled: bool;

    // sql query to grab guild config from database
    let guild_config = sqlx::query_as!(
        GuildConfig,
        "SELECT * FROM config WHERE guild_id = ?",
        guild_id
    )
    .fetch_one(&*database)
    .await.unwrap().clone();

    // send error message if the command is disabled
    if guild_config.roleconfig_userperm.unwrap() == 0 {
        enabled = false;

        let embed = serenity::CreateEmbed::default()
        .title("Command disabled!")
        .description("Please edit the configuration to enable this command.")
        .color(config.color_config.pink);

        ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await.unwrap();

        return GuildConfigCheck {config: guild_config, enabled: enabled};
    };
    enabled = true;

    // return struct with guild config and enabled information
    return GuildConfigCheck {config: guild_config, enabled: enabled};
}