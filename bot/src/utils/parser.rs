use shlex::split as shlex_split;
use std::collections::{HashMap, HashSet};

pub fn parse_args<S: AsRef<str>>(
    content: S,
) -> Result<(HashMap<String, String>, Vec<String>, HashSet<String>), String> {
    let content = content.as_ref().to_string();
    let tokens: Vec<String> = match shlex_split(content.as_str()) {
        Some(some) => some,
        None => return Err(String::from("Missing closing quotation")),
    };
    let mut params: HashMap<String, String> = HashMap::new();
    let mut inputs: Vec<String> = Vec::new();
    let mut flags: HashSet<String> = HashSet::new();

    for i in 0..tokens.len() {
        if tokens[i].starts_with("--") {
            flags.insert(tokens[i][2..].to_string());
        } else if tokens[i].starts_with("-") {
            match tokens.get(i + 1) {
                Some(some) => params.insert(tokens[i][1..].to_string(), some.to_string()),
                None => return Err(format!("Got no value for argument '{}'", &tokens[i][1..])),
            };
        } else {
            inputs.push(tokens[i].to_string());
        }
    }
    Ok((params, inputs, flags))
}

pub fn parse_codeblocks<S: AsRef<str>>(content: S) -> Result<Vec<String>, String> {
    let content = content.as_ref().to_string();
    if content.matches("```").count() % 2 != 0 {
        return Err(String::from("Unpaired ```s"));
    }

    let (_, rest_content) = content.split_at(content.find("```").unwrap_or(0));

    let blocks: Vec<String> = rest_content
        .split("```")
        .map(|x| String::from(x))
        .collect::<Vec<String>>()[1..]
        .iter()
        .step_by(2)
        .map(|x| String::from(x))
        .collect();

    Ok(blocks)
}

pub fn parse_codeblock_lang<S: AsRef<str>>(content: S) -> Result<(String, String), String> {
    let content = content.as_ref().to_string();
    match content.split_once("\n") {
        Some(some) => Ok((some.0.to_string(), some.1.to_string())),
        None => Err(String::from(
            "The correct codeblock syntax is \\`\\`\\`lang\ncode\n\\`\\`\\`",
        )),
    }
}
