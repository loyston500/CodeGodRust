mod compilers;
mod utils;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_args() {
        dbg!(utils::parser::parse_args(String::from(
            "inp1 inp2 -g value1 -h value 2 -f 'this is a long argument' --goo --foo"
        ))
        .unwrap());
    }

    #[test]
    fn test_parse_codeblocks() {
        dbg!(utils::parser::parse_codeblocks(String::from(
            "
        --foo some random stuff
        lol abcdedf ```py
        print(69)
        print('the code')
        ```
        some text that is of no use
        ```js
        console.log('some other code')
        ```
        again some extra text
        "
        ))
        .unwrap());

        dbg!(utils::parser::parse_codeblocks(String::from(
            "
        ```py
        print(69)
        print('the code')```
        some text that is of no use
        ```js
        console.log('some other code')```
        again some extra text
        "
        ))
        .unwrap());
    }

    #[test]
    fn test_parse_codeblock_lang() {
        dbg!(utils::parser::parse_codeblock_lang(String::from("py\nprint(nice)")).unwrap());
    }

    #[test]
    fn test_rextester_parse_langs() {
        dbg!(compilers::rextester::client::parse_langs(String::from(
            "
        # this is a comment
        py | python | pyth 69
        js |javascript|    cringe 53
        rust | best|aaa 43
        # this is another commment.

        "
        ))
        .unwrap());
    }

    #[test]
    fn test_rextester_parse_lang_args() {
        dbg!(compilers::rextester::client::parse_lang_args(String::from(
            "
        1 -foo bar /555
        2         -bar -5
        3 66 -- 5
        # this is a comment.
        "
        ))
        .unwrap());
    }
}
