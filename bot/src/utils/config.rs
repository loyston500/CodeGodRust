use crate::utils::misc::get_file_content;
use lazy_static::lazy_static;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Bot {
    pub name: Option<String>,
    pub prefix: String,
    pub brief: Option<String>,
    pub token_variable: String,
}

#[derive(Deserialize)]
pub struct CodeExecutor {
    pub trigger_emoji: String,
}

#[derive(Deserialize)]
pub struct CompilerRextester {
    pub enabled: bool,
    pub langs_path: String,
    pub args_path: String,
}

#[derive(Deserialize)]
pub struct CompilerTio {
    pub enabled: bool,
    pub langs_path: String,
    pub aliases_path: String,
}

#[derive(Deserialize)]
pub struct CompilerWandbox {
    pub enabled: bool,
    pub langs_path: String,
    pub aliases_path: String,
}

#[derive(Deserialize)]
pub struct MongdbTriggerEmoji {
    pub enabled: bool,
    pub uri_variable: String,
    pub database: String,
    pub collection: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub bot: Bot,
    pub code_executor: CodeExecutor,
    pub mongodb_trigger_emoji: MongdbTriggerEmoji,
    pub compiler_rextester: CompilerRextester,
    pub compiler_tio: CompilerTio,
    pub compiler_wandbox: CompilerWandbox,
}

lazy_static! {
    pub static ref CONFIG: Config = toml::from_str(
        get_file_content("config.toml")
            .expect("Cannot load the config file. The file maybe missing!")
            .as_str()
    )
    .expect("Error reading the config file. The file maybe corrupt.");
}
