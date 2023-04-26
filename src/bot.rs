use anyhow::{Result,Context};
use twitchchat::{UserConfig, AsyncRunner,
    messages::{Commands, Privmsg},
    runner::{Status},
    twitch::color::Color,
};

use std::{str::FromStr};
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug, Clone)]
pub struct TwitchChannel {
    name: String,
    color: Color
}

impl TwitchChannel {
    pub fn new(name: String, color: Option<Color>) -> TwitchChannel {

        let mut chan_color: Color = Color::from_str("#FFFFFF").unwrap();
        let mut rng = thread_rng();

        if color == None {
            // let color_options = [ Color::from_str("Blue").unwrap(), Color::from_str("BlueViolet").unwrap(), Color::from_str("CadetBlue").unwrap(), Color::from_str("Chocolate").unwrap(), Color::from_str("Coral").unwrap(), Color::from_str("DodgerBlue").unwrap(), Color::from_str("Firebrick").unwrap(), Color::from_str("GoldenRod").unwrap(), Color::from_str("Green").unwrap(), Color::from_str("HotPink").unwrap(), Color::from_str("OrangeRed").unwrap(), Color::from_str("Red").unwrap(), Color::from_str("SeaGreen").unwrap(), Color::from_str("SpringGreen").unwrap(), Color::from_str("YellowGreen").unwrap(), ];
            let color_options = [ "#0000FF", "#8A2BE2", "#5F9EA0", "#D2691E", "#FF7F50", "#1E90FF", "#B22222", "#DAA520", "#008000", "#FF69B4", "#FF4500", "#FF0000", "#2E8B57", "#00FF7F", "#ADFF2F", ];
            chan_color = Color::from_str(color_options.choose(&mut rng).unwrap().clone()).unwrap();
        }
        TwitchChannel { name: name, color: chan_color}
    }
}

#[derive(Debug)]
pub struct Bot {
    user_config: UserConfig,
    channels: Vec<TwitchChannel>,
}

impl Bot {

    pub fn add_channel(&mut self, channel: TwitchChannel) -> anyhow::Result<(), String> {
        let channels = self.channels.iter();

        for c in channels {
            if c.name == channel.name {
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

async fn run(user_config: &UserConfig, channels: &Vec<TwitchChannel>) -> anyhow::Result<()> {

    let connector = twitchchat::connector::smol::Connector::twitch()?;
    let mut runner = AsyncRunner::connect(connector, &user_config).await?;

    for channel in channels {
        if let Err(err) = runner.join(&channel.name).await {
            eprintln!("error joining channel {}: {}", &channel.name, err);
        } else {
            println!("Joined {}", channel.name);
            dbg!(channel);
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
                    _ => println!("{}", term_string(&channels, &pm))
                }
                
            },
            Status::Eof => { println!("EOF"); break },
            Status::Quit => println!("Bye"),
            Status::Message(..) => continue,
        }
    }

    Ok(())
}

pub fn term_string(chan_list: &Vec<TwitchChannel>, pm: &Privmsg) -> String {
    let mut channel = TwitchChannel{ name: String::from("na"), color: Color::from_str("#FFFFFF").unwrap()};
    let target_chan = pm.channel().split_at(1).1;
    for chan in chan_list {
        if chan.name == target_chan {
            channel = chan.clone();
            break;
        } 
    }
    let colored_channel = std::format!("[#\x1b[38;2;{};{};{}m{}\x1b[0m]", channel.color.rgb.0,channel.color.rgb.1,channel.color.rgb.2, channel.name);
    let c = pm.color().unwrap_or_else(|| Color::from_str("#FFFFFF").unwrap());
    // std::format!("{}> \x1b[38;2;{};{};{}m{}\x1b[0m: {}", pm.channel(), c.rgb.0, c.rgb.1, c.rgb.2, pm.name(), pm.data())
    std::format!("{} \x1b[38;2;{};{};{}m{}\x1b[0m: {}", colored_channel, c.rgb.0, c.rgb.1, c.rgb.2, pm.name(), pm.data())
}

fn get_env(key: &str) -> anyhow::Result<String> {
    std::env::var(key).with_context(|| format!("you need to set the {} env var.", key))
}

fn get_channels() -> anyhow::Result<Vec<TwitchChannel>> {
    let channels = get_env("TWITCH_CHANNELS")?
        .split(",")
        .map(|channel_name| {
            TwitchChannel::new(String::from(channel_name), None)
        })
        .collect();
    Ok(channels)
}

fn get_username() -> anyhow::Result<String> {
    get_env("TWITCH_USERNAME")
}

fn get_token() -> anyhow::Result<String> {
    get_env("TWITCH_TOKEN")
}