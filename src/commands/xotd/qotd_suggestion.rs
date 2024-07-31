use ::serenity::all::{ChannelId, CreateMessage};

use crate::{common::{command_enable_checks::check_xotd, framework::{Context, Error}}, CONFIG, POOL};

use poise::{serenity_prelude as serenity, CreateReply};

/// Suggest a question for QOTD.
#[poise::command(slash_command, prefix_command)]
pub async fn qotd_suggestion(
    ctx: Context<'_>,
    #[description = "Suggestion"] suggestion: String,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;

    // init variables
    let database = POOL.get().unwrap().clone();
    let config = CONFIG.read().unwrap().clone();   
    let guild_id = i64::from(ctx.guild().unwrap().id);
    let command_config = check_xotd(ctx, guild_id, database, config.clone()).await;

    // stop command if the command is disabled
    if command_config.enabled == false { return Ok(()) };

    // embed suggestion
    let suggestion_embed = serenity::CreateEmbed::default()
        .title("QOTD Suggestion")
        .description(format!("{}", suggestion))
        .color(config.color_config.primary);

    // send suggestion to channel
    let channel_id = command_config.config.qotd_suggestions_channel.unwrap().parse::<u64>().unwrap();
    ChannelId::from(channel_id).send_message(ctx.http(), CreateMessage::default().embed(suggestion_embed)).await?;
    ctx.send(CreateReply::default().content("Your suggestion has been sent!").reply(true).ephemeral(true)).await?;

    Ok(())
}