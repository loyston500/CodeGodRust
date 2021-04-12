# CodeGodRust [UNFINISHED]
A discord bot that can run your codes.

## Why Rust?
Firstly, It's because I wanted to learn it, and also handle a project simultaneously.
Secondly, the performance you get is incredible considering how easy is to write rust.
Thirdly, everyone loves it.

## How am I supposed to use it???
All you need to do is to write your code inside a codeblock (make sure you label the codeblock with a valid language) and then react to it with ▶ (the default emoji)

## How do I run my own instance of it??
Well, you just need to run these commands (assuming that you already have rust installed)
```bash
# clones the repo
git clone https://github.com/loyston500/CodeGodRust

# changes your directory to the required forder (super important to be in this folder because the bot needs to access some of the necessary files)
cd CodeGodRust/bot/src

# sets your bot token as an environment variable
export DISCORD_TOKEN='your token'

# builds and runs the bot 
cargo run --release
```

## Kk but, what about changing the prefix???
Just modify `config.toml` (located in CodeGodRust/bot/src) according to your needs 🙂

