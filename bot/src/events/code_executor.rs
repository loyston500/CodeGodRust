use std::collections::HashSet;
use std::time::Duration;

use serenity::async_trait;
use serenity::client::Context;
use serenity::client::EventHandler;
use serenity::model::channel::Message;
use serenity::model::channel::MessageReference;
use serenity::model::channel::Reaction;
use serenity::model::channel::ReactionType;
use serenity::model::id::EmojiId;

// use lazy_static::lazy_static;

use crate::compilers::rextester;
use crate::compilers::tio;
use crate::utils;

use crate::Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn reaction_add(&self, ctx: Context, reaction: Reaction) {
        // the thing why CodeGod was initially made for.

        // I know you are here to see what tf this guy just wrote
        // So... let's see!

        // exits if unable to fetch the reaction info.

        let reactor = match reaction.user(&ctx).await {
            Ok(ok) => ok,
            Err(_) => return,
        };

        // checks if the user is a bot or not.

        if reactor.bot {
            return;
        }

        // checks if the reaction is the required emoji or not.

        if dbg!(reaction.emoji.as_data()) != "✅" {
            return;
        }

        // same thing what we did before.

        let message = match reaction.message(&ctx).await {
            Ok(ok) => ok,
            Err(_) => return,
        };

        // spilt the message as args and the rest of the content.

        let mut content: &String = &message.content;
        let (args, rest_content) = content.split_at(content.find("```").unwrap_or(0));

        // bruh just try to understand it all by yourself,
        // I can't explain you all this lol

        let (params, inputs, flags) = match utils::parser::parse_args(args) {
            Ok(ok) => ok,
            Err(err) => {
                let _ = message
                    .channel_id
                    .say(&ctx, format!("**ArgumentParserError:** {}", err))
                    .await;
                return;
            }
        };

        let codeblocks = match utils::parser::parse_codeblocks(rest_content) {
            Ok(ok) => ok,
            Err(err) => {
                let _ = message
                    .channel_id
                    .say(&ctx, format!("**CodeblockParserError:** {}", err))
                    .await;
                return;
            }
        };

        // idk why I did this thing below but, it's cool ngl

        #[rustfmt::skip]
        let n = if codeblocks.len() > 1 {
            let _ = message
                .channel_id
                .say(
                    &ctx,
                    format!(
                        "which one do you want to run? Enter number from 1 to {}.",
                        codeblocks.len()
                    ),
                )
                .await;
            match message
                .author
                .await_reply(&ctx)
                .timeout(Duration::from_secs(30))
                .await
            {
                Some(answer) => {
                    ({
                        match answer.content.parse::<usize>() {
                            Ok(ok) => {
                                if ok > codeblocks.len() {
                                    let _ = message
                                        .channel_id
                                        .say(
                                            &ctx,
                                            format!(
                                                "Sorry, that's too high{}. Cancelled.",
                                                if ok == 20 + 49 {
                                                    " but, nice anyway"
                                                } else {
                                                    ""
                                                }
                                            ),
                                        )
                                        .await;
                                    return;
                                }
                                ok
                            }
                            Err(_) => {
                                let _ = message
                                    .channel_id
                                    .say(&ctx, "Sorry, that's an invalid response. Cancelled.")
                                    .await;
                                return;
                            }
                        }
                    } - 1)
                }
                None => 0,
            }
        } else {0};

        let (lang, code) = match utils::parser::parse_codeblock_lang(&codeblocks[n]) {
            Ok(ok) => ok,
            Err(_) => {
                let _ = message
                    .channel_id
                    .say(&ctx, "Looks like you forgot to mention the language lol.")
                    .await;
                return;
            }
        };

        // WARNING some bad code below

        let input = params
            .get(&String::from("i"))
            .unwrap_or(&String::from(""))
            .clone(); // try to avoid clone

        let compiler = params
            .get(&String::from("c"))
            .unwrap_or(&String::from("rex"))
            .clone(); // here as well

        let mut final_output = String::from("...");
        let mut stats = String::from("...");
        let mut _error = String::from("");

        // end of bad code

        // send an emoji reaction as a confirmation.

        let running = &message
            .react(
                &ctx,
                ReactionType::Custom {
                    animated: true,
                    id: EmojiId::from(797304934151487558),
                    name: Some(String::from("RunningCodeGrey")),
                },
            )
            .await;

        match compiler.as_str() {
            // selecting the compiler, NOTE: Time to spam F because from April 10th
            // rextester.com api will be available ONLY for patreons. F
            "rex" | "rextester" | "rextester.com" => {
                let lang_id: &usize = match rextester::client::LANG_ID_MAP.get(&lang) {
                    Some(some) => some,
                    None => {
                        let _ = message
                            .channel_id
                            .say(&ctx, format!("The language `{}` cannot be compiled.", lang))
                            .await;
                        return;
                    }
                };
                let lang_arg = rextester::client::ID_ARG_MAP
                    .get(lang_id)
                    .unwrap_or(&String::from(""))
                    .clone();

                let response = match rextester::client::post_request(
                    code,
                    lang_id.to_string(),
                    input,
                    lang_arg,
                )
                .await
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        let _ = message
                            .channel_id
                            .say(&ctx, format!("**ClientError:** {}", err))
                            .await;
                        return;
                    }
                };

                let json = match rextester::client::response_to_json(response).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        let _ = message
                            .channel_id
                            .say(&ctx, format!("**ClientError:** {}", err))
                            .await;
                        return;
                    }
                };
                let _result = json.Result.unwrap_or(String::from(""));
                let _error = json.Errors.unwrap_or(String::from(""));
                let _warnings = json.Warnings.unwrap_or(String::from(""));

                stats = json.Stats.unwrap_or(String::from(""));

                // all of that for this
                final_output = format!("{}{}{}", &_result, &_warnings, &_error);
            }

            // THE END
            "tio" | "tio.run" => {
                if !tio::client::LANGS.contains(&lang) {
                    let _ = message
                        .channel_id
                        .say(&ctx, format!("The language `{}` cannot be compiled.", lang))
                        .await;
                    return;
                }

                // creates a request string exactly how tio needs it.
                // And yes I'm doing unpack because I don't really think
                // it'll turn out to be an Err.

                let tio_req = tio::client::make_request_string(&lang, &code, &input).unwrap();

                // now compress the thing because tio only accepts zlib compressed bytes

                let compressed = tio::client::zlib_compress(tio_req).unwrap();

                let response = match tio::client::post_request(dbg!(compressed
                    [2..(compressed.len() - 4)]
                    .to_vec()))
                .await
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        dbg!(format!("failed to do the tio request {}", err));
                        return;
                    }
                };

                if response.status() != 200 {
                    let _ = message
                        .channel_id
                        .say(&ctx, "**ServerError:** returned non Ok status.")
                        .await;
                    return;
                }

                let decompressed =
                    tio::client::gzip_decompress(response.bytes().await.unwrap().to_vec()).unwrap();
                let pre = decompressed
                    .split(&decompressed[..16] /* <- is the token btw */)
                    .collect::<Vec<&str>>();

                // WARNING a better implementation is needed

                let pre_slice = &mut pre[1..(pre.len() - 1)].to_vec();

                let err_stats = pre_slice
                    .pop()
                    .unwrap()
                    .rsplitn(2, "\n\n")
                    .collect::<Vec<&str>>();
                dbg!(&err_stats);

                if err_stats.len() == 1 {
                    stats = err_stats[0].to_string();
                } else {
                    _error = err_stats[1].to_string();
                    stats = err_stats[0].to_string();
                }

                // ________________________________

                final_output = format!("{}{}", &pre_slice.pop().unwrap(), &_error);

                dbg!(pre_slice);
            }

            _ => {
                // if the compiler is not supported

                let _ = message.channel_id.say(&ctx, "invalid compiler name").await;
                return;
            }
        };

        let desc: String = format!(
            "```{}\n{}```",
            params
                .get(&String::from("h"))
                .unwrap_or(&String::from("css"))
                .clone(),
            if final_output.len() < 1950 {
                &final_output[..]
            } else {
                &final_output[..1950]
            }
        );

        if flags.contains(&String::from("clean")) {
            let _ = message.channel_id.say(&ctx, desc).await;
        } else {
            let _ = message
                .channel_id
                .send_message(&ctx, |m| {
                    m.embed(|e| {
                        e.description(desc);
                        e.footer(|f| {
                            f.text(stats);
                            f
                        });
                        e.color(if (_error == String::from("")) {
                            0x00BA9C
                        } else {
                            0xFFCF24
                        });
                        e
                    });
                    m
                })
                .await;
        }

        match running {
            Ok(ok) => {
                let _ = ok.delete_all(&ctx).await;
            }
            Err(_) => {}
        };
    }
}
