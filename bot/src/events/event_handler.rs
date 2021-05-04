use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::model::channel::Reaction;

use crate::events;
use crate::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        events::code_executor::reaction_handler(ctx, reaction)
            .await
            .unwrap();
    }
}
