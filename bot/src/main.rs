// The Rust rewrite of CodeGod.

#![feature(str_split_once)]
use std::env;

use serenity::async_trait;
use serenity::client::Client;
//use serenity::client::bridge::gateway::{ShardId, ShardManager};
use serenity::framework::standard::{
    StandardFramework,
    macros::{
        group
    }
};
mod commands;
use commands::misc::*;

#[group]
#[commands(ping)]
struct General;

use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    name: String,
    prefix: String,
    bot_token_var: String
}

mod utils;
mod compilers;

struct Handler;
mod events;

#[tokio::main]
async fn main() {
    let config_content: String = match utils::misc::get_file_content("config.toml") {
        Ok(ok) => ok,
        Err(_) => {
            println!("Cannot load the config file. The file maybe missing!");
            return;
        }
    };

    let config: Config = match toml::from_str(config_content.as_str()) {
        Ok(ok) => ok,
        Err(_) => {
            println!("Error reading the config file. The file maybe corrupt.");
            return;
        }
    };

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&config.prefix)) // the default bot prefix
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    let token = env::var(&config.bot_token_var).expect("Token not found :(");
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
