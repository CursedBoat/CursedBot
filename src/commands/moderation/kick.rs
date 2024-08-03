use crate::{common::framework::{Context, Error}, CONFIG};
use poise::{serenity_prelude as serenity, CreateReply};

// kick a user
#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS"
)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "User to kick"] user: serenity::User,
) -> Result<(), Error>{
    // init variables
    let config = CONFIG.read().unwrap().clone();
    let guild = ctx.guild().unwrap().id;

    let kicked = guild.kick(ctx.http(), &user).await;

    let embed: serenity::CreateEmbed;
    match kicked {
        Ok(_) => {
            embed = serenity::CreateEmbed::default()
                .title("KICKED")
                .description(format!("Kicked user ``{}``", &user.name))
                .color(config.color_config.primary);
        },
        Err(e) => {
            embed = serenity::CreateEmbed::default()
                .title("Could not kick user")
                .description(format!("Error \n``{}``", e))
                .color(config.color_config.primary);
        }
    }

    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await?;
    Ok(()) 
}