use anyhow::{Result,Context};
use twitchchat::{UserConfig, AsyncRunner,
    messages::{Commands, Privmsg},
    runner::{Status},
    twitch::color::Color,
};

use std::{str::FromStr};

pub struct Bot {
    user_config: UserConfig,
    channels: Vec<String>,
}

impl Bot {
    pub fn add_channel(&mut self, channel: String) {
        self.channels.push(channel)
    }
    pub fn new(config: Option<UserConfig>) -> Bot{
        match config {
            None => {
                let user_config = UserConfig::builder()
                    .anonymous()
                    .enable_all_capabilities()
                    .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

                let channels = vec![];
                Bot { user_config: user_config, channels: channels }
            }
            Some(_) => {
                let user_config = UserConfig::builder()
                    .name(get_username().unwrap_or_else(|e| {panic!("Could not get username: {}", e)}))
                    .token(get_token().unwrap_or_else(|e| {panic!("Could not get token: {}", e)}))
                    .enable_all_capabilities()
                    .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

                let channels = vec![];
                Bot { user_config: user_config, channels: channels }
            }
        }
    }
    pub fn init_anonymous(self) -> Bot {
        let user_config = UserConfig::builder()
            .anonymous()
            .enable_all_capabilities()
            .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

        let channels = vec![];
        Bot { user_config: user_config, channels: channels }
     }

    pub fn init_user(&mut self) -> Bot {
        let user_config = UserConfig::builder()
            .name(get_username().unwrap_or_else(|e| {panic!("Could not get username: {}", e)}))
            .token(get_token().unwrap_or_else(|e| {panic!("Could not get token: {}", e)}))
            .enable_all_capabilities()
            .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

        let channels = vec![];
        Bot { user_config: user_config, channels: channels }
    }
    pub fn run(self) -> Result<()> {
        println!("Hello, world!");
        smol::block_on(async move {run(&self.user_config, &self.channels).await })
    }
}

async fn run(user_config: &UserConfig, channels: &[String]) -> anyhow::Result<()> {

    use std::collections::VecDeque;
    const BATCH_SIZE: usize = 2;
    let mut message_queue: VecDeque<Privmsg> = VecDeque::new();
    println!("VecDeque initial length: {}", message_queue.len());
    let connector = twitchchat::connector::smol::Connector::twitch()?;
    let mut runner = AsyncRunner::connect(connector, &user_config).await?;

    for channel in channels {
        println!("joining channel {}", channel);
        if let Err(err) = runner.join(&channel).await {
            eprintln!("error joining channel {}: {}", &channel, err);
        }
        //runner has joined the channels requested
    }

    let mut writer = runner.writer();
    let quit = runner.quit_handle();
    loop {
        match runner.next_message().await? {
            Status::Message(Commands::Privmsg(pm)) => {
                // message_queue.push_back(pm.clone());
                println!("{}", term_string(&pm));
                
            },
            Status::Eof => { println!("EOF"); break },
            Status::Quit => println!("Bye"),
            Status::Message(..) => continue,
        }
        if message_queue.len() == BATCH_SIZE {
            //filtering
            println!("*flush*");
            while message_queue.len() > 0 {
                println!("{}", term_string(&message_queue.pop_front().unwrap()));
            }
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