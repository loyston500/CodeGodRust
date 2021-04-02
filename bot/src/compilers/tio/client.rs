use crate::utils::misc::get_file_content;

use std::char;
use std::collections::HashSet;
use std::io::prelude::*;

use flate2::read::GzDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use lazy_static::lazy_static;
use reqwest::{header, Client, Response};

pub fn make_request_string<L: AsRef<str>, C: AsRef<str>, I: AsRef<str>>(
    lang: L,
    code: C,
    input: I,
) -> Result<String, String> {
    let code = code.as_ref();
    let lang = lang.as_ref();
    let input = input.as_ref();

    let mut req_string = "Vlang\x001\x00".to_owned()
        + lang
        + "\x00F.code.tio\x00"
        + code.len().to_string().as_str()
        + "\x00"
        + code
        + "\x00";

    if input != "" {
        req_string += &("F.input.tio\x00".to_owned()
            + input.len().to_string().as_str()
            + "\x00"
            + input
            + "\x00R");
    } else {
        req_string += "R";
    }

    Ok(req_string.to_string())
}

pub fn zlib_compress<S: AsRef<str>>(req_string: S) -> Result<Vec<u8>, String> {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    match e.write_all(req_string.as_ref().as_bytes()) {
        Ok(_) => (),
        Err(_) => return Err(String::from("failed to write to the encoder")),
    };
    match e.finish() {
        Ok(ok) => Ok(ok),
        Err(_) => Err(String::from("failed to compress")),
    }
}

pub fn gzip_decompress(bytes: Vec<u8>) -> Result<String, String> {
    let mut decoder = GzDecoder::new(&bytes[..]);
    let mut string = String::new();
    match decoder.read_to_string(&mut string) {
        Ok(ok) => (),
        Err(_) => return Err(String::from("unable to decompress")),
    };
    Ok(string)
}

pub async fn post_request(bytes: Vec<u8>) -> Result<Response, String> {
    match Client::new()
        .post("https://tio.run/cgi-bin/run/api/")
        .header("Connection", "keep-alive")
        .body(bytes)
        .send()
        .await
    {
        Ok(ok) => Ok(ok),
        Err(_) => Err(String::from("failed to get response from the api.")),
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

lazy_static! {
    pub static ref LANGS: HashSet<String> =
        parse_langs(get_file_content("compilers/tio/langs.txt").unwrap()).unwrap();
}
