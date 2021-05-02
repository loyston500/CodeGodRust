use std::time::{Duration, SystemTime};

use serenity::client::Context;
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::channel::Message;

use crate::Database;

/// tells the ping (heart beat)
#[command]
pub async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    let start = SystemTime::now();
    let mut message = msg.channel_id.say(&ctx.http, "Ping?").await?;
    let end = SystemTime::now();
    match end.duration_since(start) {
        Ok(difference) => {
            message
                .edit(ctx, |m| {
                    m.content(format!("Pong!\nlatency: {:?}", difference))
                })
                .await?
        }
        Err(_) => message.edit(ctx, |m| m.content("Pong!")).await?,
    };
    Ok(())
}

/// sets the trigger emoji of the server.
#[command]
pub async fn setemoji(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.guild_id.is_none() {
        let _ = msg
            .channel_id
            .say(&ctx, "This command can only be used in a guild/server.")
            .await?;
        return Ok(());
    }

    let mut resp = msg
        .channel_id
        .say(
            &ctx,
            "React to this message with the emoji you want to set.",
        )
        .await?;

    if let Some(reaction) = resp
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(30))
        .author_id(msg.author.id)
        .message_id(resp.id)
        .channel_id(resp.channel_id)
        .await
    {
        let emoji = reaction.as_inner_ref().emoji.as_data();
        let result = {
            let data = ctx.data.read().await;

            data.get::<Database>()
                .expect("Error: database is not initialized properlly.")
                .clone()
                .read()
                .await
                .set_emoji(msg.guild_id.unwrap().0, &emoji)
                .await
        };

        if let Ok(_) = result {
            resp.edit(&ctx, |m| {
                m.content(format!("Successfully set the trigger emoji to {}.", emoji))
            })
            .await?;
        } else {
            resp.edit(&ctx, |m| {
                m.content("An error occured while setting the emoji, please try again.")
            })
            .await?;
        }
    } else {
        resp.edit(&ctx, |m| m.content("You did not react in time."))
            .await?;
    }

    Ok(())
}
