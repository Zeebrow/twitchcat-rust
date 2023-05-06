pub mod cli;
use config_rs::conf::parser::conf;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use crate::bot::TwitchChannel;
use twitchchat::twitch::color::Color;
use std::{env, fs};
use std::path::Path;
use std::str::FromStr;


/// config values order of precedence:
/// ----------------------------------
/// 1. cli arguments (if implemented)
/// 1. environment variable (if implemented)
/// 1. config file (implemented where possible)
/// 1. default value (implemented where possible)
/// 

const TWITCH_USERNAME_ENV_VAR: &str = "TWITCH_USERNAME";
const TWITCH_TOKEN_ENV_VAR: &str = "TWITCH_TOKEN";
const TWITCH_CHANNELS_ENV_VAR: &str = "TWITCH_CHANNELS";


#[derive(Debug, Serialize, Deserialize)]
pub struct TwitchBotConfig {
    pub app: AppConfig,
}

impl TwitchBotConfig {
    pub fn get_config() -> TwitchBotConfig {
        let config_file = Path::new("/home/zeebrow/rust/twitchcat-rs/botconfig.yaml");
        let f = fs::File::open(config_file).expect("failed to open config file");
        let yaml: TwitchBotConfig = serde_yaml::from_reader(f).unwrap_or_else(|e| {
            panic!("failed to parse config {}", e)
        });
        yaml
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    credentials: Credentials,
    pub channels: Vec<ConfigChannel>,
}

impl Default for AppConfig {
    fn default() -> AppConfig {
        AppConfig {
            credentials: Credentials::default(),
            channels: vec![
                ConfigChannel {
                    name: String::from("museun"),
                    color: String::from("#FF0000"),
                },
                ConfigChannel {
                    name: String::from("strager"),
                    color: String::from("#400080"),
                },
            ]
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Credentials {
    pub username: String,
    #[serde(skip)]
    pub token: String,
}

impl Default for Credentials {
    fn default() -> Credentials {
        Credentials {
            username: env::var(TWITCH_USERNAME_ENV_VAR).unwrap_or_else(|_e| String::from("justinfan1234")),
            token: env::var(TWITCH_TOKEN_ENV_VAR).unwrap_or_else(|_e| String::from("justinfan1234")),
        }
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigChannel {
    pub name: String,
    pub color: String,
}

impl ConfigChannel {
    pub fn to_twitch_channel(self) -> TwitchChannel {
        let color_str = String::to_owned(&self.color);
        let chan = TwitchChannel::new(self.name, Some(Color::from_str(&color_str).unwrap_or_else(|_| {
                println!("bad color value for channel");
                Color::from_str("#FFFFFF").unwrap()
            })));
        chan
    }
}
