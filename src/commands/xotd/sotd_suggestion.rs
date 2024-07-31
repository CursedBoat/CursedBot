use poise::ChoiceParameter;
use poise::serenity_prelude as serenity;
use poise::CreateReply;
use ::serenity::all::ChannelId;
use ::serenity::all::CreateMessage;
use crate::common::command_enable_checks::check_xotd;
use crate::{common::{database::get_pool, framework::{Context, Error}, structs::SpotifyOembed}, CONFIG};
use regex::Regex;

#[derive(Debug, poise::ChoiceParameter)]
pub enum DspChoice {
    #[name = "Spotify"]
    Spotify,
    #[name = "YouTube"]
    YouTube,
    #[name = "Other"]
    Other,
}

/// Send SOTD/SOTW Suggestions.
#[poise::command(slash_command, prefix_command)]
pub async fn sotd_suggestion(
    ctx: Context<'_>,
    #[description = "Link of the song"] suggestion_link: String,
    #[description = "Pick which DSP the link points to."] suggestion_dsp: DspChoice,
    #[description = "Artist name"] artist_name: String,
    #[description = "Song name"] song_name: String,
) -> Result<(), Error>{
    ctx.defer_ephemeral().await?;

    // init variables
    let database = get_pool();
    let config = CONFIG.read().unwrap().clone();
    let guild_id = i64::from(ctx.guild().unwrap().id);
    let command_config = check_xotd(ctx, guild_id, database, config.clone()).await;
    let mut thumbnail_url: String = "https://media1.tenor.com/m/To5SDNV04mcAAAAC/ryo-yamada-bocchi.gif".to_string();
    
    // do nothing if command isn't enabled
    if command_config.enabled == false { return Ok(()) };

    // match dsp names and set thumbnails
    match suggestion_dsp.name() {
        "Spotify" => {
            // use spotify OEmbed api
            let url = format!("https://embed.spotify.com/oembed?url={}", suggestion_link);
            let body: SpotifyOembed = ureq::get(&url)
                .set("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/104.0.5112.79 Safari/537.36")
                .call()?
                .into_json()?;

            // set thumbnail url
            thumbnail_url = body.thumbnail_url;
        },
        "YouTube" => {
            // regex that i totally wrote myself
            let regex = Regex::new(
                r#"(?:https?://)?(?:www\.)?youtube\.com/(?:watch\?v=|embed\/|v\/|shorts\/)?([^\?&\"\'>]+)|(?:https?://)?youtu\.be\/([^\?&\"\'>]+)"#
            ).unwrap();

            // grab regex captures
            let caps = regex.captures(&suggestion_link.as_str()).unwrap();
            let youtube_id = match caps.get(1) {
                Some(matched) => {Some(matched.as_str().to_owned())},
                None => {caps.get(2).map(|matched| matched.as_str().to_owned())},
            };

            // set thumbnail url
            thumbnail_url = format!("https://i.ytimg.com/vi_webp/{}/maxresdefault.webp", youtube_id.unwrap());
        },
        _ => {},
    }

    // embed suggestion
    let suggestion_embed = serenity::CreateEmbed::default()
        .title("SOTD/W Suggestion")
        .url(suggestion_link)
        .description(format!("{} - {}", artist_name, song_name))
        .thumbnail(thumbnail_url)
        .color(config.color_config.primary);

    // send suggestion to channel
    let channel_id = command_config.config.sotd_suggestions_channel.unwrap().parse::<u64>().unwrap();
    ChannelId::from(channel_id).send_message(ctx.http(), CreateMessage::default().embed(suggestion_embed)).await?;
    ctx.send(CreateReply::default().content("Your suggestion has been sent!").reply(true).ephemeral(true)).await?;

    Ok(())
}