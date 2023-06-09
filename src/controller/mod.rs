use std::{str::FromStr, sync::mpsc::{Receiver, Sender}};
use std::io::Write;
use twitchchat::twitch::color::Color;
use std::sync::mpsc;
use crate::bot::{TwitchChannel, Bot,  colored_string};


enum ControllerCommand {
    QUIT,
    HELP,
    CONFIG,
    START,
    STOP,
    UNKNWON,
}

#[derive(Debug)]
pub struct UnknownCommandError{}

impl std::error::Error for UnknownCommandError {
    fn description(&self) -> &str {
        "failed to parse bool"
    }
}
impl std::fmt::Display for UnknownCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        "provided string was not `true` or `false`".fmt(f)
    }
}
impl<'a> FromStr for ControllerCommand {
    // type Err = Box<dyn std::error::Error>;
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.eq("help") { Ok(Self::HELP) }
        else if s.eq("quit") { Ok(Self::QUIT) }
        else if s.eq("start") { Ok(Self::START) }
        else if s.eq("config") { Ok(Self::CONFIG) }
        else if s.eq("stop") { Ok(Self::STOP) }
        else { 
            Err(format!("not a command: {}", s).into())
        }
    }
}

pub fn prompt(ps: &str) -> String {
    let mut line = String::new();
    print!("{}", ps);
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut line).expect("could not read a line from stdin");
    line.trim().to_string()
}

pub struct BotController {
    pub comm_in: Receiver<String>,
    pub comm_out: Sender<String>,
}

impl BotController {
    pub fn new() -> BotController {
        let (tx, rx) = mpsc::channel();
        BotController {comm_in: rx, comm_out: tx}
    }

    pub fn get_prompt(&self, ps: &str) {
        let mut cfg = twitchchat::twitch::UserConfig::builder().anonymous().enable_all_capabilities().build().unwrap();
        let mut bot = Bot::new();
        loop {
            let input = prompt(&ps);

            self.comm_out.send(input.clone()).unwrap();

            let cmd = ControllerCommand::from_str(input.as_str()).unwrap_or_else(|_| {
                ControllerCommand::UNKNWON
            });
            match cmd {
                ControllerCommand::HELP => {
                    println!("'quit' quit");
                    println!("'help'");
                    println!("'start'");
                    println!("'config' => add a channel");
                },
                ControllerCommand::QUIT => { println!("bye"); break; },
                ControllerCommand::START => {
                    print!("starting with {} channels: ", bot.channels.len());
                    // let mut bot = Bot::new();
                    for channel in &bot.channels {
                        print!("{},", channel);
                    }
                    // let mut bot = Bot::new(Some(cfg.clone()));
                    // for channel in &channels {
                    //     bot.add_channel(channel);
                    // }
                    bot.run().unwrap_or_else(|e| {
                        eprintln!("{}", e);
                        ()
                    });
                    println!("done");
                },
                ControllerCommand::CONFIG => {
                    let input_channel = prompt("add channel (blank for no)?");
                    let c = Color::from_str("Blue").unwrap();
                    if !input_channel.eq("") {
                        let ch = TwitchChannel::new(input_channel, Some(c));
                        let s = colored_string(&ch.name, c);
                        println!("=> {}", s);
                        dbg!(&bot.channels);
                        bot.add_channel(ch).unwrap_or_else(|e| {
                            println!("failed to add channel: {}", e);
                        });
                        dbg!(&bot.channels);

                        // channels.push(ch);
                    }

                },
                ControllerCommand::STOP => { println!("bye"); break; },
                ControllerCommand::UNKNWON => {},
            }
        }
    }
}
