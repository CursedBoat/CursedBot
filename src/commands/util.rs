use poise::{serenity_prelude as serenity, CreateReply};
use crate::{common:: framework::{Context, Error}, CONFIG, STARTTIME};

/// Get the bot's uptime.
#[poise::command(slash_command, prefix_command)]
pub async fn uptime(
    ctx: Context<'_>,
) -> Result<(), Error>{
    let start_time = STARTTIME.read().unwrap().clone();
    let uptime_seconds = start_time/1000;

    ctx.reply(format!(
        "Server online since <t:{}:R>", uptime_seconds
    )).await?;

    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn embed_test(
    ctx: Context<'_>,
) -> Result<(), Error>{
    let config = CONFIG.read().unwrap().clone();

    let embed = serenity::CreateEmbed::default()
        .title("Sample Embed")
        .description("helo")
        .color(config.color_config.primary);

    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await?;
    Ok(())
}