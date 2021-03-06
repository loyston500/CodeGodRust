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

use crate::compilers::{rextester, tio, wandbox};
use crate::utils;
use crate::utils::config::CONFIG;
use crate::Database;
use std::error::Error;

pub async fn reaction_handler(ctx: Context, reaction: Reaction) -> Result<(), Box<dyn Error>> {
    // the thing why CodeGod was initially made for.

    // I know you are here to see what tf this guy just wrote
    // So... let's see!

    // exits if unable to fetch the reaction info.

    let reactor = reaction.user(&ctx).await?;

    // checks if the user is a bot or not.

    if reactor.bot {
        return Ok(());
    }

    // yeah you can disable the database

    let emoji = if reaction.guild_id.is_some() && CONFIG.mongodb_trigger_emoji.enabled {
        let data = ctx.data.read().await;
        data.get::<Database>()
            .expect("Error: database is not initialized properlly.")
            .clone()
            .read()
            .await
            .get_emoji(reaction.guild_id.unwrap().0)
            .await
            .unwrap_or(CONFIG.code_executor.trigger_emoji.clone())
    } else {
        CONFIG.code_executor.trigger_emoji.clone()
    };
    // checks if the reaction is the required emoji or not.

    if reaction.emoji.as_data() != emoji {
        return Ok(());
    }

    // same thing what we did before.

    let message = reaction.message(&ctx).await?;

    // this macro saved my life lol
    macro_rules! send {
        ($content:expr) => {
            let _ = message.channel_id.say(&ctx, $content).await?;
        };
    }

    macro_rules! rem_reactions {
        (ok $reac:ident) => {
            match $reac {
                Ok(ok) => {
                    let _ = ok.delete_all(&ctx).await;
                }
                Err(_) => (),
            };
        };

        ($reac:ident) => {
            let _ = $reac.delete_all(&ctx).await;
        };
    }

    let mut content: &String = &message.content;
    
    // rem_reactions!(reaction);

    // split the message as args and rest of the content

    let (args, rest_content) = content.split_at(content.find("```").unwrap_or(0));

    // bruh just try to understand it all by yourself,
    // I can't explain you all this lol

    let codeblocks = match utils::parser::parse_codeblocks(rest_content) {
        Ok(ok) => ok,
        Err(err) => {
            send!(format!("Your codeblock syntax is invalid. {}.", err));
            return Ok(());
        }
    };

    if codeblocks.len() == 0 {
        return Ok(());
    }
    
    
    let (params, inputs, flags) = match utils::parser::parse_args(args) {
        Ok(ok) => ok,
        Err(err) => {
            send!(format!("Your argument syntax is invalid. {}.", err));
            return Ok(());
        }
    };

    // idk why I did this thing below but, it's cool ngl

    #[rustfmt::skip]
    let n = if codeblocks.len() > 1 {
            send!(format!("Which one do you want to run? Enter number from 1 to {}.", codeblocks.len()));
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
                                    send!(format!(
                                        "Sorry, that's too high{}. Cancelled.",
                                        if ok == 20 + 49 {
                                            " but, nice anyway"
                                        } else {
                                           ""
                                        }
                                    ));
                                    return Ok(());
                                }
                                ok
                            }
                            Err(_) => {
                                send!("Sorry, that's an invalid response. Cancelled.");
                                return Ok(());
                            }
                        }
                    } - 1)
                }
                None => 0,
            }
        } else {0};

    let (lang, code) = match utils::parser::parse_codeblock_lang(&codeblocks[n]) {
        Ok(ok) => ok,
        Err(err) => {
            send!(format!("Your codeblock syntax is invalid lol. {}", err));
            return Ok(());
        }
    };

    rem_reactions!(reaction);

    // WARNING some bad code below

    let mut lang = params.get(&String::from("l")).unwrap_or(&lang).clone(); // oof

    let input = params
        .get(&String::from("i"))
        .unwrap_or(&String::from(""))
        .clone(); // try to avoid clone

    let compiler = params
        .get(&String::from("c"))
        .unwrap_or(&String::from("tio"))
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
                    send!(format!("Compiler `{}` does not support running `{}` codes. You may try using a different compiler.", &compiler, lang));
                    rem_reactions!(ok running);
                    return Ok(());
                }
            };

            let lang_arg = rextester::client::ID_ARG_MAP
                .get(lang_id)
                .unwrap_or(&String::from(""))
                .clone();

            let response =
                match rextester::client::post_request(code, lang_id.to_string(), input, lang_arg)
                    .await
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        send!(format!("**ServerError:** {}", err));
                        rem_reactions!(ok running);
                        return Ok(());
                    }
                };

            let json = match rextester::client::response_to_json(response).await {
                Ok(ok) => ok,
                Err(err) => {
                    send!(format!("**ServerError:** {}", err));
                    rem_reactions!(ok running);
                    return Ok(());
                }
            };

            let _result = json.Result.unwrap_or(String::from(""));
            let _error = json.Errors.unwrap_or(String::from(""));
            let _warnings = json.Warnings.unwrap_or(String::from(""));

            stats = json.Stats.unwrap_or(String::from(""));

            // all of that for this
            final_output = format!("{}{}{}", &_result, &_warnings, &_error);
        }

        "tio" | "tio.run" => {
            if !tio::client::LANGS.contains(&lang) {
                match tio::client::ALIASES.get(&lang) {
                    Some(some) => {
                        lang = some.clone();
                        // dbg!(&lang);
                    }
                    None => {
                        send!(format!("Compiler `{}` does not support running `{}` codes. You may try using a different compiler.", &compiler, lang));
                        rem_reactions!(ok running);
                        return Ok(());
                    }
                }
            }
            // dbg!(&lang);
            // creates a request string exactly how tio needs it.
            // And yes I'm doing unpack because I don't really think
            // it'll turn out to be an Err.

            let tio_req = tio::client::make_request_string(&lang, &code, &input).unwrap();

            // now compress the thing because tio only accepts zlib compressed bytes

            let compressed = tio::client::zlib_compress(tio_req).unwrap();

            let response =
                match tio::client::post_request(compressed[2..(compressed.len() - 4)].to_vec())
                    .await
                {
                    Ok(ok) => ok,
                    Err(err) => {
                        send!(format!(
                            "**ServerError:** Failed to do the tio request {}",
                            err
                        ));
                        rem_reactions!(ok running);
                        return Ok(());
                    }
                };

            if response.status() != 200 {
                send!("**ServerError:** Returned non Ok status.");
                rem_reactions!(ok running);
                return Ok(());
            }

            let decompressed =
                tio::client::gzip_decompress(response.bytes().await.unwrap().to_vec()).unwrap();
            let pre = decompressed
                .split(&decompressed[..16] /* <- is the token btw */)
                .collect::<Vec<&str>>();

            // WARNING a better implementation is needed

            let pre_slice = &mut pre[1..(pre.len() - 1)].to_vec();
            // dbg!(&pre_slice);

            let err_stats = pre_slice
                .pop()
                .unwrap()
                .rsplitn(2, "\n\n")
                .collect::<Vec<&str>>();

            if err_stats.len() == 1 {
                stats = err_stats[0].to_string();
            } else {
                _error = err_stats[1].to_string();
                stats = err_stats[0].to_string();
            }

            stats = stats.trim().replace("\n", ", ");

            // ________________________________

            final_output = format!("{}{}", &pre_slice.pop().unwrap(), &_error);
        }

        "wand" | "wandbox" => {
            if !wandbox::client::LANGS.contains(&lang) {
                match wandbox::client::ALIASES.get(&lang) {
                    Some(some) => {
                        lang = some.clone();
                        // dbg!(&lang);
                    }
                    None => {
                        send!(format!("Compiler `{}` does not support running `{}` codes. You may try using a different compiler.", &compiler, lang));
                        rem_reactions!(ok running);
                        return Ok(());
                    }
                }
            }

            let response = match wandbox::client::post_request(code, lang, "", "", false).await {
                Ok(ok) => ok,
                Err(err) => {
                    send!(format!("**ServerError:** {}", err));
                    rem_reactions!(ok running);
                    return Ok(());
                }
            };

            let json = match wandbox::client::response_to_json(response).await {
                Ok(ok) => ok,
                Err(err) => {
                    send!(format!("**ServerError:** {}", err));
                    rem_reactions!(ok running);
                    return Ok(());
                }
            };

            let status_code = json.status.unwrap_or(String::from("0"));
            final_output = format!(
                "{}{}",
                json.compiler_message.unwrap_or(String::from("")),
                json.program_message.unwrap_or(String::from(""))
            );
            _error = if status_code == "0" {
                String::from("")
            } else {
                String::from("err")
            };
            stats = format!("Status code: {}", status_code);
        }

        _ => {
            // if the compiler is not supported

            send!("Sorry, that's an invalid compiler name. Cancelled.");
            rem_reactions!(ok running);
            return Ok(());
        }
    };

    let desc: String = format!(
        "```{}\n{}```",
        params
            .get(&String::from("h"))
            .unwrap_or(&String::from("css"))
            .clone(),
        if final_output.len() == 0 {
            &"NO OUTPUT"
        } else if final_output.len() < 1950 {
            &final_output[..]
        } else {
            &final_output[..1950]
        }
    );

    // https://tenor.com/view/jeremy-clarkson-sometimes-my-genius-almost-frightening-driving-car-ride-gif-16463163

    let result = match (if flags.contains(&String::from("file")) {
        // creates a file out of final_output which contains
        // the whole thing that the api returned.
        let file_name = format!(
            "output.{}",
            params
                .get(&String::from("e"))
                .unwrap_or(&String::from("txt"))
        );

        message
            .channel_id
            .send_files(
                &ctx,
                vec![(final_output.as_bytes(), file_name.as_str())],
                |f| f,
            )
            .await
    } else if flags.contains(&String::from("clean")) {
        // just include --clean as a flag in your message and
        // you'll get your result without the embed
        message.channel_id.say(&ctx, desc).await
    } else {
        message
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title("Jump to message");
                    e.url(&message.link());
                    e.description(desc);
                    e.footer(|f| {
                        f.text(stats);
                        f
                    });

                    e.color(if (_error == "") { 0x00BA9C } else { 0xFFCF24 });
                    e
                });
                m
            })
            .await
    }) {
        Ok(ok) => ok,
        Err(_) => {
            send!("**ClientError:** Failed to send the message.");
            rem_reactions!(ok running);
            return Ok(());
        }
    };

    // Now since the message with code's output is sent,
    // the running code reaction can be removed.

    rem_reactions!(ok running);

    // now react to the output message with ???????
    // so when the code's author clicks it, the output gets deleted

    let trash_bin = result
        .react(&ctx, ReactionType::Unicode(String::from("???????")))
        .await;

    // trying `if let` syntax for the first time tho

    if let Some(some_reaction) = result
        .await_reaction(&ctx)
        .timeout(Duration::from_secs(30))
        .author_id(message.author.id)
        .message_id(result.id)
        .channel_id(result.channel_id)
        .await
    {
        // not sure why I gotta do all of this for getting the emoji as an str.

        let emoji = &some_reaction.as_inner_ref().emoji.as_data();

        if emoji == "???????" {
            let _ = result.delete(&ctx).await;
        }
    } else {
        rem_reactions!(ok trash_bin);
    };

    Ok(())
}
