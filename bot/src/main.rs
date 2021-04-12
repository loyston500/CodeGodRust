// The Rust rewrite of CodeGod.

#![feature(str_split_once)]
use std::env;

use serenity::async_trait;
use serenity::client::Client;
//use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serde::Deserialize;
use serenity::framework::standard::{macros::group, StandardFramework};
use lazy_static::lazy_static;

mod commands;
use commands::misc::*;

#[group]
#[commands(ping)]
struct General;

#[derive(Deserialize)]
struct Config {
    name: String,
    prefix: String,
    trigger_emoji: String,
    bot_token_var: String,
}

mod compilers;
mod utils;

struct Handler;
mod events;

lazy_static! {
    static ref CONFIG: Config = 
        toml::from_str(
            utils::misc::get_file_content("config.toml")
                .expect("Cannot load the config file. The file maybe missing!")
                .as_str()
        ).expect("Error reading the config file. The file maybe corrupt.");
}

#[tokio::main]
async fn main() {
    let token = env::var(&CONFIG.bot_token_var)
        .expect("Token not found :( maybe try doing `export DISCORD_TOKEN='token'`");

    // Showcase
    println!("validating stuff...");

    dbg!(compilers::rextester::client::LANG_ID_MAP.len());
    dbg!(compilers::rextester::client::ID_ARG_MAP.len());

    dbg!(compilers::tio::client::LANGS.len());
    dbg!(compilers::tio::client::ALIASES.len());

    dbg!(compilers::wandbox::client::LANGS.len());
    dbg!(compilers::wandbox::client::ALIASES.len());

    println!("starting the bot...");

    if let Ok(logo_art) = utils::misc::get_file_content("assets/logo_art_rainbow.txt") {
        println!("{}", logo_art);
    }

    // __________

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&CONFIG.prefix)) // the default bot prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start().await {
        Ok(_) => println!("The bot's running."),
        Err(why) => println!("An error occurred while running the client: {:?}", why),
    }
}
