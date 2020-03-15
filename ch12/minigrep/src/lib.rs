#![allow(dead_code, unused_variables, unused_doc_comments)]

use std::error::Error;
use std::fs;
use std::env;

pub struct Config { 
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone(); 

        /**
         * [참고! - command에 따라 env값이 어떻게 할당되나?]
         * $ CASE_INSENSITIVE=1 cargo run to poem.txt -> case_sensitive == true
         * $ cargo run to poem.txt -> case_sensitive == false
         */
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}

/**
 * Box<dyn Error>는 함수가 Error 트레이트를 구현하는 타입을 리턴하지만, 리턴될 값의 타입을 특정하지 않는다.
 * 이렇게 하면 여러 에러 상황에서 각기 다른 타입을 리턴할 수 있는 유연성을 확보할 수 있다.ExactSizeIterator
 */
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }
    Ok(())
}


pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}



// ====== test


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )

    }
}
