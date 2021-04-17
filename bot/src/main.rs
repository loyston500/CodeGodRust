// The Rust rewrite of CodeGod.

#![feature(str_split_once)]
use std::env;

use serenity::async_trait;
use serenity::client::Client;
//use serenity::client::bridge::gateway::{ShardId, ShardManager};
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::framework::standard::{macros::group, StandardFramework};

mod utils;
use utils::config::CONFIG;

mod compilers;

// setup commands
mod commands;
use commands::misc::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;
mod events;

#[tokio::main]
async fn main() {
    let token = env::var(&CONFIG.bot.token_variable)
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
        .configure(|c| c.prefix(&CONFIG.bot.prefix)) // the default bot prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
