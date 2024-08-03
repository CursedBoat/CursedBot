use std::time::{SystemTime, UNIX_EPOCH};

use crate::{common::framework::{Context, Error}, CONFIG};
use poise::{serenity_prelude as serenity, CreateReply};
use ::serenity::all::Timestamp;

// timeout a user
#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS"
)]
pub async fn timeout(
    ctx: Context<'_>,
    #[description = "User to timeout"] user: serenity::User,
    #[description = "Hours to timeout user for"] duration: i64,
) -> Result<(), Error>{
    // init variables
    let config = CONFIG.read().unwrap().clone();
    let guild = ctx.guild().unwrap().id;

    let systemtime = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    let time = Timestamp::from_unix_timestamp(systemtime + duration*120).unwrap();
    let timedout = guild.member(ctx.http(), &user.id).await?.disable_communication_until_datetime(ctx.http(), time).await;

    let embed: serenity::CreateEmbed;
    match timedout {
        Ok(_) => {
            embed = serenity::CreateEmbed::default()
                .title("TIMED OUT")
                .description(format!("Timed out user ``{}``", &user.name))
                .color(config.color_config.primary);
        },
        Err(e) => {
            embed = serenity::CreateEmbed::default()
                .title("Could not timeout user")
                .description(format!("Error \n``{}``", e))
                .color(config.color_config.primary);
        }
    }

    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await?;
    Ok(()) 
}