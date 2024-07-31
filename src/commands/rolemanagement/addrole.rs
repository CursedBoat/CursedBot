use std::str::FromStr;

use crate::{common::{color_conversion::Color, command_enable_checks::check_role, framework::{Context, Error}}, CONFIG, POOL};
use poise::serenity_prelude as serenity;
use ::serenity::all::EditRole;

/// Adds a decorator role to the user.
#[poise::command(
    slash_command, 
    prefix_command,
    required_permissions = "MANAGE_MESSAGES | MANAGE_THREADS",
)]
pub async fn add_decorator_role(
    ctx: Context<'_>,
    #[description = "Name of the role"] role_name: String,
    #[description = "User of the role"] user: serenity::User,
    #[description = "Hex color. (Without #)"] hex: String,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;
    
    // init variables
    let database = POOL.get().unwrap().clone();
    let config = CONFIG.read().unwrap().clone();
    let color = Color::from_str(&hex);
    let guild_id = i64::from(ctx.guild().unwrap().id); 
    let command_config = check_role(ctx, guild_id, database.clone(), config.clone()).await;

    // do nothing if command isn't enabled
    if command_config.enabled == false { return Ok(()) };

    // return error and exit command if color is invalid
    if let Err(e) = color {
        ctx.reply(e).await?; return Ok(())
    }
    let color = color.unwrap();

    // create role with color
    let role_params = EditRole::new()
        .name(role_name)
        .colour((
            color.color_vec[0], // r
            color.color_vec[1], // g
            color.color_vec[2], // b
        ))
        .hoist(true);
    let role = ctx.guild_id().unwrap().create_role(ctx.http(), role_params).await?;

    // add role to user
    ctx.http().add_member_role(ctx.guild_id().unwrap(), user.id, role.id, Some("Added by moderator")).await?;

    // sql query for adding the info to the database
    let sql_query = format!(
        "INSERT INTO '{}' (username, role_id) VALUES ($1, $2)",
        guild_id
    );

    // add role info to database
    sqlx::query(&sql_query)
        .bind(user.name)
        .bind(role.id.to_string())
        .execute(&*database)
        .await?;

    ctx.reply("Added role, and added user to the database").await?;
    Ok(())
}

/// Adds the database entry for the decorator role.
#[poise::command(
    slash_command, 
    prefix_command,
    required_permissions = "MANAGE_MESSAGES | MANAGE_THREADS",
)]
pub async fn add_decorator_role_db_only(
    ctx: Context<'_>,
    #[description = "The role"] role: serenity::Role,
    #[description = "User of the role"] user: serenity::User,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;
    
    // init variables
    let database = POOL.get().unwrap().clone();
    let config = CONFIG.read().unwrap().clone();
    let guild_id = i64::from(ctx.guild().unwrap().id); 
    let command_config = check_role(ctx, guild_id, database.clone(), config.clone()).await;

    // do nothing if command isn't enabled
    if command_config.enabled == false { return Ok(()) };

    // sql query for adding the info to the database
    let sql_query = format!(
        "INSERT INTO '{}' (username, role_id) VALUES ($1, $2)",
        guild_id
    );

    // add role info to database
    sqlx::query(&sql_query)
        .bind(user.name)
        .bind(role.id.to_string())
        .execute(&*database)
        .await?;

    ctx.reply("Added user to the database").await?;
    Ok(())
}