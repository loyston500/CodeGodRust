use crate::utils::misc::get_file_content;

use std::collections::{HashMap, HashSet};

use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde;
use serde::{Deserialize, Serialize};

// well defined structure of the API's json request
#[derive(Default, Serialize)]
#[allow(dead_code)]
pub struct ApiRequest {
    pub compiler: String,
    pub code: String,
    pub options: String,
    #[serde(rename = "compiler-option-raw")]
    pub compiler_options_raw: String,
    pub save: bool,
}

// well defined structure of the API's json response.
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct ApiResponse {
    pub status: Option<String>,
    pub compiler_message: Option<String>,
    pub program_message: Option<String>,
    pub compiler_error: Option<String>,
    pub program_output: Option<String>,
}

pub async fn post_request<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>, V: AsRef<str>>(
    code: S,
    compiler: T,
    options: U,
    compiler_options_raw: V,
    save: bool,
) -> Result<Response, String> {
    match Client::new()
        .post("https://wandbox.org/api/compile.json")
        .header("content-type", "application/json")
        .json(&ApiRequest {
            code: code.as_ref().to_string(),
            compiler: compiler.as_ref().to_string(),
            options: options.as_ref().to_string(),
            compiler_options_raw: compiler_options_raw.as_ref().to_string(),
            save,
        })
        .send()
        .await
    {
        Ok(ok) => Ok(ok),
        Err(_) => Err(String::from("failed to get response from the api.")),
    }
}

pub async fn response_to_json(response: Response) -> Result<ApiResponse, String> {
    match response.json::<ApiResponse>().await {
        Ok(ok) => Ok(ok),
        Err(_) => Err(String::from("the api sent a bad request.")),
    }
}

pub fn parse_langs(content: String) -> Result<HashSet<String>, String> {
    let mut langs = HashSet::new();

    for (n, line_) in content.lines().enumerate() {
        let line = line_.trim();
        if (!line.starts_with("#")) && (line != "") {
            langs.insert(line.to_string());
        }
    }

    Ok(langs)
}

pub fn parse_aliases(content: String) -> Result<HashMap<String, String>, String> {
    let mut aliases = HashMap::new();

    for (n, line_) in content.lines().enumerate() {
        let line = line_.trim();

        if (!line.starts_with("#")) && (line != "") {
            let mut tokens = line.split_whitespace().collect::<Vec<&str>>();
            let lang = tokens.remove(0).to_string();

            if tokens.len() == 0 {
                return Err(format!(
                    "Error at line {}, no alias to set for lang `{}`",
                    n + 1,
                    lang
                ));
            }

            for token in tokens {
                aliases.insert(token.to_string().clone(), lang.clone());
            }
        }
    }

    Ok(aliases)
}

lazy_static! {
    pub static ref LANGS: HashSet<String> = parse_langs(
        get_file_content("compilers/wandbox/langs.txt").expect("failed to read wandbox langs.txt")
    )
    .unwrap();
    
    pub static ref ALIASES: HashMap<String, String> = parse_aliases(
        get_file_content("compilers/wandbox/aliases.txt")
            .expect("failed to read wandbox aliases.txt")
    )
    .unwrap();
}
