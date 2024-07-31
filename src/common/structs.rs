use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub client_config: ClientConfig,
    pub bot_config: BotConfig,
    pub color_config: ColorConfig,
    pub spotify_config: SpotifyConfig,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SpotifyConfig {
    pub client_id: String,
    pub client_secret: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClientConfig {
    pub token: String,
    pub app_id: String,
    pub test_guild_id: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BotConfig {
    pub prefix: String,
    pub decorator_role_index: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ColorConfig {
    pub primary: i32,
    pub secondary: i32,
    pub blue: i32,
    pub pink: i32,
    pub green: i32
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRoleConfig {
    pub username: String,
    pub role_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GuildConfig {
    pub guild_id: Option<i64>,
    pub qotd_channel: Option<String>,
    pub sotd_channel: Option<String>,
    pub qotd_suggestions_channel: Option<String>,
    pub sotd_suggestions_channel: Option<String>,
    pub xotd_enable: Option<i64>,
    pub roleconfig_userperm: Option<i64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GuildConfigCheck {
    pub config: GuildConfig,
    pub enabled: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SpotifyOembed {
    pub html: String,
    pub iframe_url: String,
    pub width: i64,
    pub height: i64,
    pub version: String,
    pub provider_name: String,
    pub provider_url: String,
    pub title: String,
    pub r#type: String,
    pub thumbnail_url: String,
    pub thumbnail_width: i64,
    pub thumbnail_height: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DharMannJson {
    pub person: Vec<String>,
    pub action: Vec<String>,
    pub closer: Vec<String>,
}