use std::str::FromStr;

use crate::{common::{color_conversion::Color, command_enable_checks::check_role, framework::{Context, Error}}, CONFIG, POOL};
use ::serenity::all::{EditRole, RoleId};
use sqlx::Row;

/// Deletes decorator role from DB and removes it from the user.
#[poise::command(
    slash_command, 
    prefix_command,
)]
pub async fn update_role(
    ctx: Context<'_>,
    #[description = "Name of the role"] name_arg: Option<String>,
    #[description = "Hex code of the color. (Without the #)"] color_arg: Option<String>,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;
    
    // init variables
    let database = POOL.get().unwrap().clone();
    let config = CONFIG.read().unwrap().clone();
    let guild_id = i64::from(ctx.guild().unwrap().id); 
    let command_config = check_role(ctx, guild_id, database.clone(), config.clone()).await;
    let username = &ctx.author().name;

    // do nothing if command isn't enabled
    if command_config.enabled == false { return Ok(()) };

    // sql query for adding the info to the database
    let sql_query = format!(
        "SELECT * FROM '{}' WHERE username = $1",
        guild_id
    );

    // get role info from database
    let query = sqlx::query(&sql_query)
        .bind(username)
        .fetch_one(&*database)
        .await?;
    let role_id: String = query.get(1);

    // get specific role from the guild
    let guild_roles = ctx.guild_id().unwrap().roles(ctx.http()).await?;
    let guild_role = guild_roles.get(&mut RoleId::from_str(&role_id).unwrap());

    // set config variables
    let name: String;
    let color: Vec<u8>;

    // role name
    match name_arg {
        None => { name = guild_role.unwrap().clone().name; },
        Some(val) => {name = val }
    }

    // role color
    match color_arg {
        None => { 
            color = vec![
                guild_role.unwrap().colour.r(), 
                guild_role.unwrap().colour.g(), 
                guild_role.unwrap().colour.b(),
            ] 
        },
        Some(val) => {
            let rgb = Color::from_str(&val).unwrap();
            color = vec![rgb.color_vec[0], rgb.color_vec[1], rgb.color_vec[2]]
        }
    }

    // role builder
    let updated_role = EditRole::new()
        .name(name)
        .colour((
            color[0], // r
            color[1], // g
            color[2], // b
        ));

    // update role
    ctx.guild_id().unwrap().edit_role(
        ctx.http(), 
        RoleId::from_str(&role_id).unwrap(), 
        updated_role
    )
    .await?;

    // send confirmation message
    ctx.reply("Updated role.").await?;
    Ok(())
}