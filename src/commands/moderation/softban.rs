use crate::{common::framework::{Context, Error}, CONFIG};
use poise::{serenity_prelude as serenity, CreateReply};

// kick a user, but delete their past messages
#[poise::command(
    slash_command,
    required_permissions = "BAN_MEMBERS"
)]
pub async fn softban(
    ctx: Context<'_>,
    #[description = "User to softban"] user: serenity::User,
    #[description = "Number of days to delete messages"]
    #[choices(1, 2, 3, 4, 5, 6, 7)]
    dmd: u8,
) -> Result<(), Error>{
    // init variables
    let config = CONFIG.read().unwrap().clone();
    let guild = ctx.guild().unwrap().id;

    let banned = guild.ban(ctx.http(), &user, dmd).await;
    guild.unban(ctx.http(), &user.id).await?;

    let embed: serenity::CreateEmbed;
    match banned {
        Ok(_) => {
            embed = serenity::CreateEmbed::default()
                .title("BANNED")
                .description(format!("Soft-banned user ``{}``", &user.name))
                .color(config.color_config.primary);
        },
        Err(e) => {
            embed = serenity::CreateEmbed::default()
                .title("Could not softban user")
                .description(format!("Error \n``{}``", e))
                .color(config.color_config.primary);
        }
    }

    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await?;
    Ok(()) 
}