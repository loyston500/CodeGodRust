use std::env;
use std::error::Error;

use lazy_static::lazy_static;
use mongodb::bson::{self, doc, Bson, Document};
use mongodb::Collection;
use mongodb::{options::ClientOptions, Client};

use tokio;

use crate::utils::config::CONFIG;

#[derive(Clone, Debug)]
pub struct MongodbTriggerEmojis {
    pub client: Client,
    pub collection: Collection,
}

impl MongodbTriggerEmojis {
    pub async fn init<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>>(
        uri: S,
        database_name: T,
        collection_name: U,
    ) -> Result<Self, Box<dyn Error>> {
        let client = Client::with_uri_str(uri.as_ref()).await?;

        let collection = client
            .database(database_name.as_ref())
            .collection::<Document>(collection_name.as_ref());

        Ok(Self { client, collection })
    }

    pub async fn get_emoji(&self, guild_id: u64) -> Option<String> {
        if let Ok(Some(bson)) = self.collection.find_one(doc! {"_id": guild_id}, None).await {
            if let Some(value) = bson.get("emoji") {
                return Some(value.as_str().unwrap().to_string());
            }
        }

        None
    }

    pub async fn set_emoji<S: AsRef<str>>(&self, guild_id: u64, emoji: S) -> Result<(), ()> {
        let emoji = emoji.as_ref().to_string();
        if self.get_emoji(guild_id.clone()).await.is_none() {
            return match self
                .collection
                .insert_one(doc! {"_id": guild_id, "emoji": emoji}, None)
                .await
            {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            };
        } else {
            return match self
                .collection
                .update_one(
                    doc! { "_id": guild_id, },
                    doc! { "$set": { "emoji": emoji } },
                    None,
                )
                .await
            {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            };
        }
    }
}
