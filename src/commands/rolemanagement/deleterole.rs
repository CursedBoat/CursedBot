use std::str::FromStr;

use crate::{common::{command_enable_checks::check_role, framework::{Context, Error}}, CONFIG, POOL};
use poise::serenity_prelude as serenity;
use ::serenity::all::RoleId;
use sqlx::Row;

/// Deletes decorator role from DB and removes it from the user.
#[poise::command(
    slash_command, 
    prefix_command,
    required_permissions = "MANAGE_MESSAGES | MANAGE_THREADS",
)]
pub async fn remove_decorator_role(
    ctx: Context<'_>,
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
        "SELECT * FROM '{}' WHERE username = $1",
        guild_id
    );

    // get role info from database
    let query = sqlx::query(&sql_query)
        .bind(user.clone().name)
        .fetch_one(&*database)
        .await?;
    let role_id: String = query.get(1);

    // get specific role from the guild
    let guild_roles = ctx.guild_id().unwrap().roles(ctx.http()).await?;
    let guild_role = guild_roles.get(&mut RoleId::from_str(&role_id).unwrap());

    // delete role
    ctx.guild_id().unwrap().delete_role(ctx.http(), guild_role.unwrap()).await?;

    // delete user from database
    let sql_query_delete = format!(
        "DELETE FROM '{}' WHERE username = $1",
        guild_id
    );
    sqlx::query(&sql_query_delete)
        .bind(user.name)
        .execute(&*database)
        .await?;

    // send confirmation message
    ctx.reply("Removed role, and removed user from the database").await?;
    Ok(())
}