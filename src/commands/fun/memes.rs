use rand::Rng;
use roux::Subreddit;

use crate::common::framework::{Context, Error};
use poise::{serenity_prelude as serenity, CreateReply};

/// Returns a post from r/dankmemes or a subreddit of your choice
#[poise::command(slash_command, prefix_command)]
pub async fn memes(
    ctx: Context<'_>,
    #[description = "The subreddit to search"] subreddit: Option<String>
) -> Result<(), Error>{
    ctx.defer().await?;

    // init variables
    let subr_name: String;
    match subreddit {
        None => { subr_name = "dankmemes".to_string() }
        Some(subreddit) => { subr_name = subreddit }
    }
    let subr = Subreddit::new(&subr_name);
    let post = subr.hot(25, None).await.unwrap();
    let index = rand::thread_rng().gen_range(0..=25);
    let post_data = &post.data.children.get(index).unwrap().data;

    // nsfw check
    let subr_isnsfw = subr.about().await?.over18.unwrap();
    let post_isnsfw = post_data.over_18;
    let channel_isnsfw = ctx.guild_channel().await.unwrap().nsfw;

    // trigger if either subreddit or post is nsfw, but the channel isn't
    if (subr_isnsfw || post_isnsfw) && !channel_isnsfw {
        ctx.send(
            CreateReply::default()
            .content("This is an NSFW post or subreddit. Please only request nsfw content in nsfw channels.")
            .reply(true).ephemeral(true)
        ).await?;

        return Ok(());
    }

    let embed = serenity::CreateEmbed::default()
        .title(format!("{}", post_data.title))
        .url(format!("https://www.reddit.com/{}", post_data.permalink))
        .description(
            format!(
                "👍 ``{up}`` 💬 ``{com}``", 
                up = post_data.ups, 
                com = post_data.num_comments
            )
        )
        .image(post_data.url.clone().unwrap())
        .footer(
            serenity::CreateEmbedFooter::new(format!("Stolen from r/{}", subr_name))
        );
    
    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(false)).await?;
    Ok(())
}