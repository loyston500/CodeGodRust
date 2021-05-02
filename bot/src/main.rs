// The Rust rewrite of CodeGod.

use std::env;

use serenity::async_trait;
use serenity::client::Client;
//use serenity::client::bridge::gateway::{ShardId, ShardManager};
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::framework::standard::{macros::group, StandardFramework};

mod utils;
use utils::config::CONFIG;

mod database;
use database::trigger_emoji::MongodbTriggerEmojis;

mod compilers;

// setup commands
mod commands;
use commands::misc::*;

#[group]
#[commands(ping, setemoji)]
struct General;

struct Handler;
mod events;

struct Database;

use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::RwLock;

impl TypeMapKey for Database {
    type Value = Arc<RwLock<MongodbTriggerEmojis>>;
}

#[tokio::main]
async fn main() {
    let token = env::var(&CONFIG.bot.token_variable)
        .expect("Token not found :( maybe try doing `export DISCORD_TOKEN='token'`");

    println!("validating stuff...");

    dbg!(compilers::rextester::client::LANG_ID_MAP.len());
    dbg!(compilers::rextester::client::ID_ARG_MAP.len());

    dbg!(compilers::tio::client::LANGS.len());
    dbg!(compilers::tio::client::ALIASES.len());

    dbg!(compilers::wandbox::client::LANGS.len());
    dbg!(compilers::wandbox::client::ALIASES.len());

    println!("starting the bot...");

    // Showcase

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

    if CONFIG.mongodb_trigger_emoji.enabled {
        println!("initializing database...");
        // Open the data lock in write mode, so keys can be inserted to it.
        let mut data = client.data.write().await;

        let db = MongodbTriggerEmojis::init(
            env::var(&CONFIG.mongodb_trigger_emoji.uri_variable).expect("Mongodb uri not found"),
            &CONFIG.mongodb_trigger_emoji.database,
            &CONFIG.mongodb_trigger_emoji.collection,
        )
        .await
        .expect("Failed to initialize the database");

        dbg!(db.get_emoji(710863889029398640).await);

        data.insert::<Database>(Arc::new(RwLock::new(db)));
    }

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}
