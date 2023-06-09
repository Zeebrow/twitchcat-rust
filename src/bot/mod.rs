use log::{debug};
use anyhow::{Result,Context};
use serde::__private::de::IdentifierDeserializer;
use twitchchat::{UserConfig, AsyncRunner,
    messages::{Commands, Privmsg},
    runner::{Status},
    twitch::color::Color,
};

use std::{str::FromStr, sync::mpsc::{Receiver, Sender}, fmt::Display};
use rand::thread_rng;
use rand::seq::SliceRandom;

use std::sync::mpsc;


#[derive(Debug, Clone)]
pub struct TwitchChannel {
    pub name: String,
    pub color: Color,
    pub msg_rate: Box<i8>, /* per-second*/
}

impl Display for TwitchChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl TwitchChannel {
    pub fn new(name: String, color: Option<Color>) -> TwitchChannel {

        let mut chan_color: Color = Color::from_str("#FFFFFF").unwrap();
        let mut rng = thread_rng();
        let mut initial_msg_rate = Box::new(0);

        if color == None {
            // let color_options = [ Color::from_str("Blue").unwrap(), Color::from_str("BlueViolet").unwrap(), Color::from_str("CadetBlue").unwrap(), Color::from_str("Chocolate").unwrap(), Color::from_str("Coral").unwrap(), Color::from_str("DodgerBlue").unwrap(), Color::from_str("Firebrick").unwrap(), Color::from_str("GoldenRod").unwrap(), Color::from_str("Green").unwrap(), Color::from_str("HotPink").unwrap(), Color::from_str("OrangeRed").unwrap(), Color::from_str("Red").unwrap(), Color::from_str("SeaGreen").unwrap(), Color::from_str("SpringGreen").unwrap(), Color::from_str("YellowGreen").unwrap(), ];
            let color_options = [ "#0000FF", "#8A2BE2", "#5F9EA0", "#D2691E", "#FF7F50", "#1E90FF", "#B22222", "#DAA520", "#008000", "#FF69B4", "#FF4500", "#FF0000", "#2E8B57", "#00FF7F", "#ADFF2F", ];
            chan_color = Color::from_str(color_options.choose(&mut rng).unwrap().clone()).unwrap();
        } else {
            chan_color = color.unwrap();
        }
        TwitchChannel { name: name, color: chan_color, msg_rate: initial_msg_rate}
    }
    
}



#[derive(Debug)]
pub struct Bot {
    user_config: UserConfig,
    pub channels: Vec<TwitchChannel>,
    comms_in: Receiver<String>,
    comms_out: Sender<String>,
    running: bool,
}

impl Bot {

    pub fn add_channel(&mut self, channel: TwitchChannel) -> Result<(), String> {
        dbg!(&self.channels);
        for c in self.channels.iter() {
            if c.name == channel.name {
                return Err(String::from("already joined channel"))
            } 
        }
        dbg!(&self.channels);
        self.channels.push(channel.clone());
        dbg!(&self.channels);
        Ok(())
    }

    pub fn set_config(&mut self) {
        let user_config = UserConfig::builder()
            .name(get_username().unwrap_or_else(|e| {panic!("Could not get username: {}", e)}))
            .token(get_token().unwrap_or_else(|e| {panic!("Could not get token: {}", e)}))
            .enable_all_capabilities()
            .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});
        self.user_config = user_config;
    }

    pub fn set_channels(&mut self) {
        let channels = get_channels().unwrap_or_else(|e| {
            println!("no channels provided ({})", e);
            vec![]
        });
        self.channels = channels;
    }

    pub fn new() -> Bot{
        // set some defaults
        // let channels: Vec<TwitchChannel> = vec![TwitchChannel::new(String::from("museun"), Some(Color::from_str("Red").unwrap()) )];
        let channels = vec![];
        let user_config = UserConfig::builder()
            .anonymous()
            .enable_all_capabilities()
            .build().unwrap_or_else(|e| { panic!("Could not build config: {}", e)});

        // initialize a means of passing signals between the bot and controller
        // the bot should block while waiting for a signal from the controller
        let (comms_out, comms_in) = mpsc::channel();
        Bot { user_config: user_config, channels: channels, running: false, comms_in: comms_in, comms_out: comms_out}
    }

    pub fn run(&self) -> Result<()> {
        println!("Starting bot");
        dbg!(&self);
        let (tx, rx) = mpsc::channel();

        // @@@
        let res = smol::block_on(async move {run(&self.user_config, &self.channels, tx).await });
        // let res = run(&self.user_config, &self.channels, tx);
        // loop {
        //     let got_message = rx.try_recv();
        //     match got_message {
        //         Ok(msg) => {
        //             println!("=> [#{}] {}: {}", msg.channel(), msg.name(), msg.data())
        //         }
        //         Err(_) => {}
        //     }


        // }
        println!("exiting run");
        Ok(())
    }
}

async fn run<'a>(user_config: &UserConfig, channels: &Vec<TwitchChannel>, tx: Sender<Privmsg<'a>>) -> anyhow::Result<()> {

    let connector = twitchchat::connector::smol::ConnectorTls::twitch()?;
    let mut runner = AsyncRunner::connect(connector, &user_config).await?;

    for channel in channels {
        let did_join = runner.join(&channel.name).await;
        dbg!(&did_join);
        debug!("test");
        // if let Err(err) = runner.join(&channel.name).await {
        if let Ok(()) = did_join {
            println!("Joined #{}", channel.name);
        } else {
            println!("error joining #{}", channel.name);
            // eprintln!("error joining channel {}: {}", &channel.name, );
        }
        /*
        if let Err(err) = did_join {
            // I don't think this branch will ever get executed
            eprintln!("error joining channel {}: {}", &channel.name, err);
            println!("error joining #{}", channel.name);
        } else {
            // I want to println!("Joined {}", channel_name.term_color(channel.color))
            println!("Joined #{}", channel.name);
        }
        */
        //runner has joined the channels requested
    }

    let mut writer = runner.writer();

    loop {

            // let quit = runner.quit_handle();
            // quit.notify().await;
        match runner.next_message().await? {
            Status::Message(Commands::Privmsg(pm)) => {
                // message_queue.push_back(pm.clone());
                for chan in channels {
                    if String::from(pm.channel()) == format!("#{}", chan.name) {
                        println!("{}", print_term_string(chan, &pm));
                    }
                    tx.send(pm.clone()).unwrap();
                }
                // println!("{}", pm.channel());
            },
            Status::Eof => { println!("EOF"); break },
            Status::Quit => println!("Bye"),
            Status::Message(..) => continue,
        }
    }

    Ok(())
}

pub fn colored_string(s: &String, c: Color) -> String {
    std::format!("[#\x1b[38;2;{};{};{}m{}\x1b[0m]", c.rgb.0, c.rgb.1, c.rgb.2, &s)
}

pub fn print_term_string(channel: &TwitchChannel, pm: &Privmsg) -> String {
    let colored_channel = std::format!("[#\x1b[38;2;{};{};{}m{}\x1b[0m]", channel.color.rgb.0,channel.color.rgb.1,channel.color.rgb.2, channel.name);
    let c = pm.color().unwrap_or_else(|| Color::from_str("#FFFFFF").unwrap());
    // std::format!("{}> \x1b[38;2;{};{};{}m{}\x1b[0m: {}", pm.channel(), c.rgb.0, c.rgb.1, c.rgb.2, pm.name(), pm.data())
    std::format!("{} \x1b[38;2;{};{};{}m{}\x1b[0m: {}", colored_channel, c.rgb.0, c.rgb.1, c.rgb.2, pm.name(), pm.data())
}

pub fn term_string(chan_list: &Vec<TwitchChannel>, pm: &Privmsg) -> String {
    let mut channel = TwitchChannel{ name: String::from("na"), color: Color::from_str("#FFFFFF").unwrap(), msg_rate: Box::new(0)};
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