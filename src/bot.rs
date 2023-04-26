use anyhow::{Result,Context};
use twitchchat::{UserConfig, AsyncRunner,
    messages::{Commands, Privmsg},
    runner::{Status},
    twitch::color::Color,
};

use std::{str::FromStr};
use std::error::Error;

#[derive(Debug)]
pub struct Bot {
    user_config: UserConfig,
    channels: Vec<String>,
}

impl Bot {

    pub fn add_channel(&mut self, channel: &String) -> anyhow::Result<(), String> {
        let channels = self.channels.iter();

        for c in channels {
            if c == channel {
                return Err(String::from("already joined channel"))
            } 
        }
        self.channels.push(channel.clone());
        return Ok(())
    }

    pub fn new(config: Option<UserConfig>) -> Bot{
        let channels = get_channels().unwrap_or_else(|e| {
            println!("Could not get channels: {}", e);
            vec![]
        });
        match config {
            None => {
                let user_config = UserConfig::builder()
                    .anonymous()
                    .enable_all_capabilities()
                    .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

                // let channels = vec![];
                Bot { user_config: user_config, channels: channels }
            }
            Some(_) => {
                let user_config = UserConfig::builder()
                    .name(get_username().unwrap_or_else(|e| {panic!("Could not get username: {}", e)}))
                    .token(get_token().unwrap_or_else(|e| {panic!("Could not get token: {}", e)}))
                    .enable_all_capabilities()
                    .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

                // let channels = vec![];
                Bot { user_config: user_config, channels: channels }
            }
        }
    }

    pub fn run(self) -> Result<()> {
        println!("Starting bot");
        dbg!(&self);
        smol::block_on(async move {run(&self.user_config, &self.channels).await })
    }
}

async fn run(user_config: &UserConfig, channels: &[String]) -> anyhow::Result<()> {

    let connector = twitchchat::connector::smol::Connector::twitch()?;
    let mut runner = AsyncRunner::connect(connector, &user_config).await?;

    for channel in channels {
        if let Err(err) = runner.join(&channel).await {
            eprintln!("error joining channel {}: {}", &channel, err);
        } else {
            println!("Joined {}", channel);
        }
        //runner has joined the channels requested
    }

    let mut writer = runner.writer();
    let quit = runner.quit_handle();
    loop {
        match runner.next_message().await? {
            Status::Message(Commands::Privmsg(pm)) => {
                // message_queue.push_back(pm.clone());
                match pm.channel() {
                    _ => println!("{}", term_string(&pm))
                }
                
            },
            Status::Eof => { println!("EOF"); break },
            Status::Quit => println!("Bye"),
            Status::Message(..) => continue,
        }
    }

    Ok(())
}

pub fn term_string(pm: &Privmsg) -> String {
    let c = pm.color().unwrap_or_else(|| Color::from_str("#FFFFFF").unwrap());
    std::format!("{}> \x1b[38;2;{};{};{}m{}\x1b[0m: {}", pm.channel(), c.rgb.0, c.rgb.1, c.rgb.2, pm.name(), pm.data())
}

fn get_env(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("you need to set the {} env var.", key))
}

fn get_channels() -> anyhow::Result<Vec<String>> {
    let channels = get_env("TWITCH_CHANNELS")?
        .split(",")
        .map(ToString::to_string)
        .collect();
    Ok(channels)
}

fn get_username() -> anyhow::Result<String> {
    get_env("TWITCH_USERNAME")
}

fn get_token() -> anyhow::Result<String> {
    get_env("TWITCH_TOKEN")
}