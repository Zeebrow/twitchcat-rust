mod bot;
use crate::bot::{Bot};
use std::env;


fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let mut b = Bot::new(None);
    let channel_name = "lol_nemesis";
    b.add_channel(&String::from(channel_name)).unwrap_or_else(|e| {
        println!("{} {}", e, channel_name)
    });
    b.run()

}
