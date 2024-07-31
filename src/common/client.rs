use super::framework::Data;
use super::framework::Error;

use poise::serenity_prelude as serenity;
use serenity::prelude::SerenityError;

pub async fn cursedbot_client(
    framework: poise::Framework<Data, Error>,
    token: &str
) -> Result<poise::serenity_prelude::Client, SerenityError> {
    // set bot intents
    let intents = serenity::GatewayIntents::privileged() | serenity::GatewayIntents::all();

    // return the client
    return serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .status(serenity::OnlineStatus::DoNotDisturb)
        .activity(serenity::ActivityData::watching("over you!"))
        .await;
}