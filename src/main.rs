mod bot;
use crate::bot::{Bot};
use std::env;


fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    let mut b = Bot::new(None);
    b.add_channel(String::from("lol_nemesis"));
    b.run()

}
