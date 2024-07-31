use super::framework::Data;
use super::framework::Error;
use super::framework::Context;

use poise::serenity_prelude as serenity;
use poise::CreateReply;
use tracing::info;

// simple error handling
pub async fn on_error(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => panic!("Failed to start bot: {:?}", error),
        poise::FrameworkError::Command { error, ctx, .. } => {
            // log error to console
            info!("Error in command `{}`: {:?}", ctx.command().name, error,);

            // error embed
            let error_embed = serenity::CreateEmbed::default()
                .title("Uh oh! An error occured!")
                .field("Error", format!("```{}```", error.to_string()), false);

            // notify user about the error
            let _ = ctx.send(CreateReply::default().embed(error_embed).ephemeral(true))
                .await;
        },
        // basic error handling
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                info!("Error while handling error: {}", e)
            }
        }
    }
}

// global register command
#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}