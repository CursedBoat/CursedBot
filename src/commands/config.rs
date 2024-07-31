use poise::{ChoiceParameter, CreateReply};
use ::serenity::all::Channel;
use sqlx::query;
use poise::serenity_prelude as serenity;

use crate::{common::{database::get_pool, framework::{Context, Error}}, CONFIG};

#[derive(Debug, poise::ChoiceParameter)]
pub enum TrueFalseChoice {
    #[name = "Yes"]
    Yes,
    #[name = "No"]
    No
}

/// Configure the bot for your server.
#[poise::command(slash_command, prefix_command)]
pub async fn bot_config(
    ctx: Context<'_>,
    #[description = "Enable QOTD & SOTD Commands"] xotd_choice: TrueFalseChoice,
    #[description = "Enable role & color management"] role_choice: TrueFalseChoice,
    #[description = "QOTD Channel"] qotd_channel_id_arg: Option<Channel>,
    #[description = "SOTD Channel"] sotd_channel_id_arg: Option<Channel>,
    #[description = "QOTD Suggestions Channel (⚠ Leave blank if you don't want member suggestions!)"] qotd_sug_channel_id_arg: Option<Channel>,
    #[description = "SOTD Suggestions Channel (⚠ Leave blank if you don't want member suggestions!)"] sotd_sug_channel_id_arg: Option<Channel>,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;

    // initialize database and config
    let config = CONFIG.read().unwrap().clone();
    let database = get_pool();

    // initialize bool values
    let mut xotd_bool: i16 = 0;
    let mut role_bool: i16 = 0;

    // initialize channel ids
    let mut qotd_channel_id: Option<String> = None;
    let mut sotd_channel_id: Option<String> = None;
    let mut qotd_sug_channel_id: Option<String> = None;
    let mut sotd_sug_channel_id: Option<String> = None;
    
    // condition for bool variables
    if xotd_choice.name() == "Yes" {
        xotd_bool = 1
    }
    if role_choice.name() == "Yes" {
        role_bool = 1
    }

    // condition for channel id variables
    if !qotd_channel_id_arg.is_none() {
        qotd_channel_id = Some(qotd_channel_id_arg.unwrap().id().to_string());
    }
    
    if !sotd_channel_id_arg.is_none() {
        sotd_channel_id = Some(sotd_channel_id_arg.unwrap().id().to_string());
    }

    if !qotd_sug_channel_id_arg.is_none() {
        qotd_sug_channel_id = Some(qotd_sug_channel_id_arg.unwrap().id().to_string());
    }

    if !sotd_sug_channel_id_arg.is_none() {
        sotd_sug_channel_id = Some(sotd_sug_channel_id_arg.unwrap().id().to_string());
    }

    // sql for config
    let config_sql = format!(
        "UPDATE config
        SET xotd_enable = $1, 
            roleconfig_userperm = $2,
            qotd_channel = $3, 
            sotd_channel = $4, 
            qotd_suggestions_channel = $5, 
            sotd_suggestions_channel = $6
        WHERE guild_id = $7"
    );

    // sqlx query
    let guild_id = i64::from(ctx.guild().unwrap().id);
    query(&config_sql)
        .bind(xotd_bool)
        .bind(role_bool)
        .bind(qotd_channel_id.clone())
        .bind(sotd_channel_id.clone())
        .bind(qotd_sug_channel_id.clone())
        .bind(sotd_sug_channel_id.clone())
        .bind(guild_id)
        .execute(&*database)
        .await?;

    // embed
    let success_embed = serenity::CreateEmbed::default()
        .title("Successfully saved configuration!")
        .description("The configuration is shown below.")
        .color(config.color_config.primary)
        .field("(X)OTD", xotd_choice.name(), true)
        .field("Role Management", role_choice.name(), true)
        .field("SOTD Channel", sotd_channel_id.unwrap_or("DISABLED".to_string()), true)
        .field("QOTD Channel", qotd_channel_id.unwrap_or("DISABLED".to_string()), true)
        .field("SOTD Suggestions Channel", sotd_sug_channel_id.unwrap_or("DISABLED".to_string()), true)
        .field("QOTD Suggestions Channel", qotd_sug_channel_id.unwrap_or("DISABLED".to_string()), true);

    // confirmation message
    ctx.send(CreateReply::default().embed(success_embed).reply(true).ephemeral(true)).await?;
    Ok(())
}