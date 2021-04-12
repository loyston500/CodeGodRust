use crate::utils::misc::get_file_content;

use std::collections::HashMap;

use lazy_static::lazy_static;
use reqwest::{Client, Response};
use serde::Deserialize;

// well defined structure of the API's json response.
#[derive(Deserialize)]
#[allow(non_snake_case)]
#[allow(dead_code)]
pub struct ApiResponse {
    pub Result: Option<String>,
    pub Stats: Option<String>,
    pub Warnings: Option<String>,
    pub Errors: Option<String>,
}

pub async fn post_request<S: AsRef<str>, T: AsRef<str>, U: AsRef<str>, V: AsRef<str>>(
    code: S,
    id: T,
    input: U,
    args: V,
) -> Result<Response, String> {
    match Client::new()
        .post("https://rextester.com/rundotnet/api")
        .form(&[
            ("LanguageChoice", id.as_ref()),
            ("Program", code.as_ref()),
            ("Input", input.as_ref()),
            ("CompilerArgs", args.as_ref()),
        ])
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

pub fn parse_langs(content: String) -> Result<HashMap<String, usize>, String> {
    let mut lang_id_map: HashMap<String, usize> = HashMap::new();
    for (n, line_) in content.lines().enumerate() {
        let line = line_.trim();

        if (!line.starts_with("#")) && (line != "") {
            let line_split = line.rsplitn(2, " ").collect::<Vec<&str>>();

            if line_split.len() != 2 {
                return Err(format!("Line {}, `{}`, more than two terms", n, line_));
            }

            let id = match line_split[0].parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    return Err(format!(
                        "Line {} `{}`, cannot parse the term `{}` to an integer.",
                        n + 1,
                        line_,
                        line_split[0]
                    ))
                }
            };

            for lang in line_split[1].split("|") {
                lang_id_map.insert(lang.trim().to_string(), id.clone());
            }
        }
    }
    Ok(lang_id_map)
}

pub fn parse_lang_args(content: String) -> Result<HashMap<usize, String>, String> {
    let mut id_arg_map = HashMap::new();
    for (n, line_) in content.lines().enumerate() {
        let line = line_.trim();
        if (!line.starts_with("#")) && (line != "") {
            let line_split = line.splitn(2, " ").collect::<Vec<&str>>();

            if line_split.len() != 2 {
                return Err(format!(
                    "Line {}, `{}`, need exactly two terms here.",
                    n, line_
                ));
            }

            let id = match line_split[0].parse::<usize>() {
                Ok(ok) => ok,
                Err(_) => {
                    return Err(format!(
                        "Line {} `{}`, cannot parse the term `{}` to an integer.",
                        n + 1,
                        line_,
                        line_split[0]
                    ))
                }
            };

            let args = line_split[1].trim().to_string();
            id_arg_map.insert(id, args);
        }
    }
    Ok(id_arg_map)
}

lazy_static! {
    pub static ref LANG_ID_MAP: HashMap<String, usize> = parse_langs(
        get_file_content("compilers/rextester/langs.txt")
            .expect("failed to read rextester langs.txt")
    )
    .unwrap();
    
    pub static ref ID_ARG_MAP: HashMap<usize, String> = parse_lang_args(
        get_file_content("compilers/rextester/args.txt")
            .expect("failed to read rextester args.txt")
    )
    .unwrap();
}
