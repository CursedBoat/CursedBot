use ::serenity::all::CreateMessage;

use crate::{common::{command_enable_checks::check_xotd, framework::{Context, Error}}, CONFIG, POOL};

use poise::{serenity_prelude as serenity, CreateReply};

/// Answer a QOTD question.
#[poise::command(slash_command, prefix_command)]
pub async fn answer_qotd(
    ctx: Context<'_>,
    #[description = "Suggestion"] answer: String,
    #[description = "Add an image/screenshot to your answer"] image: Option<serenity::Attachment>,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;

    // init variables
    let database = POOL.get().unwrap().clone();
    let config = CONFIG.read().unwrap().clone();
    let guild_id = i64::from(ctx.guild().unwrap().id);
    let command_config = check_xotd(ctx, guild_id, database, config.clone()).await;

    // init channel id's and command config
    let qotd_channel_id = command_config.config.qotd_channel.unwrap();
    let channel_id = ctx.guild_channel().await.unwrap().parent_id.unwrap().to_string();

    // do nothing if command isn't enabled
    if command_config.enabled == false { return Ok(()) };

    // stop the command if the attachment isn't an image
    if !image.is_none() {
        let content_type = image.clone().unwrap().content_type.unwrap().split("/").nth(0).unwrap().to_string();

        if content_type != "image" {
            let embed = serenity::CreateEmbed::default()
                .title("Not allowed")
                .description("Please only pass an image.")
                .color(config.color_config.pink);

            ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await.unwrap();
            return Ok(());
        }
    }
    
    // stop the command if it isn't executed in a thread inside the qotd channel
    if channel_id != qotd_channel_id {
        let embed = serenity::CreateEmbed::default()
            .title("Not allowed")
            .description("Please don't use this command outside a QOTD thread.")
            .color(config.color_config.pink);

        ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await.unwrap();
        return Ok(());
    }

    // answer embed
    let embed = serenity::CreateEmbed::default()
        .title("Anonymous answer")
        .description(answer)
        .image("attachment://image.png")
        .color(config.color_config.primary);

    // send different messages based on if there is an image or not
    if !image.is_none() {
        let image_bytes = image.unwrap().download().await.unwrap();
        let file = serenity::CreateAttachment::bytes(image_bytes, "image.png");
        ctx.channel_id().send_message(ctx.http(), CreateMessage::default().embed(embed).add_file(file)).await?;
    } else {
        ctx.channel_id().send_message(ctx.http(), CreateMessage::default().embed(embed)).await?;
    }

    // send confirmation message
    ctx.send(CreateReply::default().content("Answer sent!").reply(true).ephemeral(true)).await?;
    Ok(())
}