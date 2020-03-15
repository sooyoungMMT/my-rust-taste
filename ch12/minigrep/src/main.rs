#![allow(dead_code)]
use std::env;
use std::fs;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let (query, filename) = parse_config(&args);
    // let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("인수를 구문분석하는 동안 오류가 발생했습니다: {}", err);
        process::exit(1);
    });
    
    println!("검색어: {}", config.query);
    println!("대상 파일: {}", config.filename);
    
    if let Err(e) = run(config) {
        println!("애플리케이션 에러: {}", e);
        process::exit(1);
    }
}

/**
 * Box<dyn Error>는 함수가 Error 트레이트를 구현하는 타입을 리턴하지만, 리턴될 값의 타입을 특정하지 않는다.
 * 이렇게 하면 여러 에러 상황에서 각기 다른 타입을 리턴할 수 있는 유연성을 확보할 수 있다.ExactSizeIterator
 */
fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    println!("파일 내용:\n{}", contents);
    Ok(())
}

struct Config { 
    query: String,
    filename: String
}
impl Config {
    fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            // panic!("필요한 인수가 지정되지 않았습니다.");
            return Err("필요한 인수가 지정되지 않았습니다.");
        }
        let query = args[1].clone();
        let filename = args[2].clone(); 

        Ok(Config { query, filename })
    }
}


fn parse_config(args: &[String]) -> Config {
    let query = args[1].clone();
    let filename = args[2].clone(); 

    Config { query, filename }
}