mod bot;
mod config;
use crate::bot::Bot;
use crate::config::TwitchBotConfig;
use std::env;
use log::*;
use env_logger;
use twitch_api::{helix::{self, Request, RequestGet, users::{GetUsersRequest, User}}, types, TwitchClient, twitch_oauth2::AppAccessToken};
use twitch_api::helix::streams::get_streams;
use twitch_api::twitch_oauth2::{Scope, ClientId, ClientSecret};
use reqwest;

fn main() -> Result<(), anyhow::Error> {
    // env_logger::init();
    // let client_id: ClientId = ClientId::new(env::var("TWITCH_BOT_CLIENT_ID").unwrap());
    // let client_secret: ClientSecret = ClientSecret::new(env::var("TWITCH_BOT_CLIENT_SECRET").unwrap());
    // let scopes: Vec<Scope> = vec![];
    // let token = AppAccessToken::get_app_access_token(&client, client_id, client_secret, scopes);

    // let logins: &[&types::UserNameRef] = &["justintv123".into()];
    // let request = GetUsersRequest::logins(logins);
    // let client: TwitchClient<reqwest::Client> = TwitchClient::default();
    // let response = send_http_request(request.create_request("accesstoken", "client_id")?)?;


    // let users = &["justintvfan".into()];
    // let request = get_streams::GetStreamsRequest::builder()
    //     .user_login(users[..])
    //     .build();
    // panic!("asdf");
    
    let app_config = TwitchBotConfig::get_config();
    dbg!(&app_config);
    let args: Vec<String> = env::args().collect();
    println!("{:#?}", args);
    /* note to self: doesn't need to be mutable, perhaps because Bot manages its
     * own memory? Let's go with that. */
    let b = Bot::new(None);
    let res = b.run();
    println!("exiting program");
    res
}
