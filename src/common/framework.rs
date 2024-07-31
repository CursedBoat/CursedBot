use std::time::Duration;

use serenity::all::GuildId;

use super::builtins::on_error;
use super::structs::Config;
use crate::commands::*;
use crate::events::handler::event_handler;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

pub fn cursedbot_framework(config: Config) -> poise::Framework<Data, Error> {
    // add commands
    let commands = vec![
        // builtins
        super::builtins::register(),

        // utilities
        config::bot_config(),
        util::uptime(),
        util::embed_test(),
        
        // misc
        test::ping(),
        test::convert_color(),

        // xotd
        xotd::qotd_suggestion::qotd_suggestion(),
        xotd::sotd_suggestion::sotd_suggestion(),
        xotd::answer_qotd::answer_qotd(),

        // roles
        rolemanagement::addrole::add_decorator_role(),
        rolemanagement::addrole::add_decorator_role_db_only(),
        rolemanagement::deleterole::remove_decorator_role(),
        rolemanagement::updaterole::update_role(),

        // fun
        fun::dharmann::generate_dharmann_title(),
        fun::random_song_recommendation::random_song_recommendation(),
        fun::memes::memes(),
        
    ];

    let bot_config = config.clone();

    // configure framework options
    let framework_options = poise::FrameworkOptions {
        commands: commands,
        prefix_options: poise::PrefixFrameworkOptions { 
            prefix: Some(config.bot_config.prefix.into()), 
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600)).into()), 
            ignore_bots: true, 
            ..Default::default()
        },
        on_error: |error| Box::pin(on_error(error)),
        event_handler: |ctx, event, framework, data| {
            Box::pin(event_handler(ctx, event, framework, data))
        },
        ..Default::default()
    };

    // return the built framework
    return poise::Framework::builder()
        .options(framework_options)
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(ctx, &framework.options().commands, GuildId::new(bot_config.client_config.test_guild_id))
                    .await?;
                Ok(Data {})
            })
        })
        .build();
}