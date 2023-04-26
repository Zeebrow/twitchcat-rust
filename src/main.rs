mod bot;
use crate::bot::{Bot, TwitchChannel};
use std::env;


fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let mut b = Bot::new(None);
    let channel_name = "lol_nemesis";
    b.add_channel(TwitchChannel::new(String::from(channel_name), None)).unwrap_or_else(|e| {
        println!("{} {}", e, channel_name)
    });
    b.run()

}
