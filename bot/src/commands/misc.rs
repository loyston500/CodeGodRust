use std::time::SystemTime;

use serenity::client::Context;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};


/// tells the ping (heart beat)
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = SystemTime::now(); 
    let mut message = msg.channel_id.say(&ctx.http, "Ping?").await?;
    let end = SystemTime::now();
    match end.duration_since(start) {
        Ok(difference) => message.edit(ctx, |m| m.content(format!("Pong!\nlatency: {:?}", difference))).await?,
        Err(_) => message.edit(ctx, |m| m.content("Pong!")).await?
    };
    Ok(())
}
