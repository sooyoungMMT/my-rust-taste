#![allow(dead_code, unused_variables)]

use std::error::Error;
use std::fs;
// use std::io::prelude::*;

pub struct Config { 
    pub query: String,
    pub filename: String
}
impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }
}

/**
 * Box<dyn Error>는 함수가 Error 트레이트를 구현하는 타입을 리턴하지만, 리턴될 값의 타입을 특정하지 않는다.
 * 이렇게 하면 여러 에러 상황에서 각기 다른 타입을 리턴할 수 있는 유연성을 확보할 수 있다.ExactSizeIterator
 */
pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("파일 내용:\n{}", contents);
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
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        )
    }
}
