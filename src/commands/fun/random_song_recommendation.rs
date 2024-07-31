use rand::Rng;
use regex::Regex;
use rspotify::{model::{Id, PlaylistId, TrackId}, prelude::BaseClient, ClientCredsSpotify, Credentials};
use poise::{serenity_prelude as serenity, CreateReply};
use crate::{common::framework::{Context, Error}, CONFIG};
use rand::seq::SliceRandom;

/// Get a totally random song recommendation that totally isn't from my playlist
#[poise::command(slash_command, prefix_command)]
pub async fn random_song_recommendation(
    ctx: Context<'_>,
) -> Result<(), Error>{
    ctx.defer().await?;

    // initialize config
    let config = CONFIG.read().unwrap().clone();

    // initialize spotify client
    let spotify = ClientCredsSpotify::new(
        Credentials {
            id: config.spotify_config.client_id,
            secret: Some(config.spotify_config.client_secret),
        }
    );

    // get auth token
    spotify.request_token().await.unwrap();

    // list of all the playlists
    let playlist_id_list = vec![
        "0VtLeP89Rf0P6AsyrvulMO".to_string(), // me when i
        "5STJap1c7RSsepmv9xpWbJ".to_string(), // rock/metal
        "0m3lxPv1dfM6JF55Y10x52".to_string(), // adhd
        "4uBF2YxZbXwHuyNbYecedK".to_string(), // depression
        "4B8ryBQW5yCybGmkYIePQD".to_string(), // speed
        "1k6P9qplIBRz5mTXL0s3zt".to_string(), // moodswing
        "3VwVSG6onMgg8jixnk67vU".to_string(), // porter robinson
        "63ecoCtXF4PZNqQmNit2QD".to_string(), // polyphia
        "2jYL5vzmoY7PFSJV0fZwVn".to_string(), // warm
        "52kOyJnJZKOS1MXSWT8uNv".to_string(), // jrock/jpop
    ];
    
    // choose random playlist
    let playlist_id_str = playlist_id_list.choose(&mut rand::thread_rng()).unwrap().as_str();
    let aaaa = format!("spotify:playlist:{}", playlist_id_str);

    // playlist fuckery (TODO: Add more playlists)
    let playlist_id = PlaylistId::from_uri(&aaaa).unwrap();
    let playlist = spotify.playlist(playlist_id, None, None).await?;
    let total_tracks = usize::try_from(playlist.tracks.total - 1).unwrap();
    let track_index: usize = rand::thread_rng().gen_range(0..total_tracks);

    // track fuckery
    let track_uri = &playlist.tracks.items[track_index].track.as_ref().unwrap().id().unwrap().uri();
    let track_id = TrackId::from_uri(track_uri).unwrap();
    let track = spotify.track(track_id.clone(), None).await?;

    // song embed
    let track_id_string = extract_track_id(track.id.unwrap().to_string()).unwrap().to_string();
    let embed = serenity::CreateEmbed::default()
        .title("Song Suggestion")
        .description(
            format!(
                "stream it! [[link](https://open.spotify.com/track/{id})]", 
                id = track_id_string
            )
        )
        .field("Artist(s)", track.artists.iter()
            .map(|artist| artist.name.clone())
            .collect::<Vec<String>>()
            .join(", "),    
        false)
        .field("Song name", track.name, false)
        .thumbnail(&track.album.images[0].url)
        .color(config.color_config.primary);

    // send reply
    ctx.send(CreateReply::default().embed(embed).reply(true).ephemeral(true)).await?;
    Ok(())
}

fn extract_track_id(uri: String) -> Option<String> {
    let re = Regex::new(r"spotify:track:([a-zA-Z0-9]+)").unwrap();
    re.captures(&uri).and_then(|cap| cap.get(1).map(|m| m.as_str().to_string()))
}