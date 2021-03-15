use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::model::channel::Message;
use serenity::model::channel::MessageReference;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;

use crate::utils;
use crate::compilers::rextester;

use crate::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        let reactor = match reaction.user(&ctx).await {
            Ok(ok) => ok,
            Err(_) => {return;}
        };
        // checks if the user is a bot or not.
        if reactor.bot {return;}
        // checks if the reaction is the required emoji or not.
        if dbg!(reaction.emoji.as_data()) != "✅" {return;}

        let message = match reaction.message(&ctx).await {
            Ok(ok) => ok,
            Err(_) => {return;}
        };

        let mut content: &String = &message.content;
        let (args, rest_content) = content.split_at(content.find("```").unwrap_or(0));
        
        let (params, inputs, flags) = match utils::parser::parse_args(args) {
            Ok(ok) => ok,
            Err(err) => {message.channel_id.say(&ctx, format!("**ArgumentParserError:** {}", err)).await; return;}
        };
        
        let codeblocks = match utils::parser::parse_codeblocks(rest_content) {
            Ok(ok) => ok,
            Err(err) => {message.channel_id.say(&ctx, format!("**CodeblockParserError:** {}", err)).await; return;}
        };
        
        let (lang, code) = match utils::parser::parse_codeblock_lang(&codeblocks[0]) {
            Ok(ok) => ok,
            Err(_) => {message.channel_id.say(&ctx, "Looks like you forgot to mention the language lol.").await; return;}
        };
        
        if codeblocks.len() > 1 {
            message.channel_id.say(&ctx, "I'll just assume you want to run the code from the first codeblock.").await;
        }
        
        
        let lang_id: &usize = match rextester::client::LANG_ID_MAP.get(&lang) {
            Some(some) => some,
            None => {message.channel_id.say(&ctx, format!("The language `{}` cannot be compiled.", lang)).await; return;}
        };
        
        let lang_arg = rextester::client::ID_ARG_MAP.get(lang_id).unwrap_or(&String::from("")).clone();
        
        let running = &message.react(
            &ctx, 
            ReactionType::Custom {
                animated: true, 
                id: EmojiId::from(797304934151487558), 
                name: Some(String::from("RunningCodeGrey"))
            }
        ).await;
        
        let response = match rextester::client::post_request(code, lang_id.to_string(), "", lang_arg).await {
            Ok(ok) => ok,
            Err(err) => {message.channel_id.say(&ctx, format!("**ClientError:** {}", err)).await; return;}
        };
        
        let json = match rextester::client::response_to_json(response).await {
            Ok(ok) => ok,
            Err(err) => {message.channel_id.say(&ctx, format!("**ClientError:** {}", err)).await; return;}
        };
        
        let _result: String = json.Result.unwrap_or(String::from(""));
        let _error: String = dbg!(json.Errors.unwrap_or(String::from("")));
        let _warnings: String = dbg!(json.Warnings.unwrap_or(String::from("")));
        let stats: String = json.Stats.unwrap_or(String::from(""));
        let final_output: String = format!("{}{}{}", _result, &_warnings, &_error);
        
        let desc: String = if final_output.len() < 1950 {
            format!("```css\n{}```", final_output)
        } else {
            format!("```css\n{}```", &final_output[..1950])
        };
    
        message.channel_id.send_message(&ctx, |m| {
            match message.message_reference {
                Some(some) => {m.reference_message(some);},
                None => {}
            };
            m.embed(|e| {
                e.description(desc);
                e.footer(|f| {
                    f.text(stats);
                    f
                });
                e.color(if (_error == String::from("")) {0x00BA9C} else {0xFFCF24});
                e
            });
            m
        }).await;

        match running {
            Ok(ok) => {ok.delete_all(&ctx).await;},
            Err(_) => {}
        };
    }
}
