# CursedBot

[![N|Solid](https://i.ibb.co/p4bb52B/Rust-programming-language-black-logo.png)](https://www.rust-lang.org/)

CursedBot is a cursed Discord bot.
## Features

- Sqlite database integration
- Fast & stable
- Examples for lots of Serenity & Poise features
- Commented code for ease of development
- KeepAlive Axum server if you want to run the bot on a free server host like Render

This bot was originally meant for a private server, so some of the commands (XOTD & Roles)
might not make too much sense to use in your own server.

NOTE: /memes command currently borked, will fix in a future update.

## Dependencies

CursedBot mostly depends on the following crates:

- [Tokio](https://tokio.rs/) - Epic async rust runtime
- [Axum](https://github.com/tokio-rs/axum) - Web app framework
- [Serenity](https://github.com/serenity-rs/serenity) - Discord API wrapper
- [Poise](https://github.com/serenity-rs/poise) - Advanced command framework for Serenity
- [Roux](https://docs.rs/roux/) - Reddit client
- [rSpotify](https://github.com/ramsayleung/rspotify) - Spotify client
- [sqlx](https://github.com/launchbadge/sqlx) - SQL 😭
- [Serde](https://serde.rs/) - Serializing and deserializing structs
- [Tracing](https://github.com/tokio-rs/tracing) - For logs

## Installation

1. Install rust via [Rustup](https://rustup.rs/)
2. Git clone the repository
3. Rename ``Config.json.example`` to ``Config.json`` and configure it
4. Add the ``DATABASE_URL = "sqlite://database.sqlite"`` environment variable & rename ``database.example.sqlite`` to ``database.sqlite``

```sh
cargo install sqlx-cli
cargo sqlx migrate migrations
cargo sqlx prepare

cd CursedBot
cargo build --release
cargo run
```
NOTE: You might need to install ``libssl-dev`` (``openssl`` in arch) & ``pkg-config`` (``pkgconf`` in arch) for it to compile without errors.

## License

GNU GPL 3.0