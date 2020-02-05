#![allow(dead_code, unused_variables)]
const PATH: &str = "hello2.txt";

use std::fs::File;
fn result_example() {
    let f = File::open(PATH); // it works!
                              // let f = File::open("hello.txt"); // error!!!

    let f = match f {
        // 성공하면 파일 핸들을 f에 할당한다.
        Ok(file) => file,
        Err(error) => {
            panic!("파일 열기 실패: {:?}", error);
        }
    };
    println!("f: {:?}", f);
}

use std::io::ErrorKind;
fn match_example() {
    // let f = File::open(PATH); // it works!
    let f = File::open("hello.txt"); // error!!!

    let f = match f {
        Ok(file) => file,
        Err(ref error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("파일을 생성하지 못했습니다!: {:?}", e),
            },
            other_error => panic!("파일을 열지 못했습니다: {:?}", other_error),
        },
    };
    println!("파일 열기 성공! {:?}", f);
}

fn unwrap_example() {
    // let f = File::open("hello.txt").unwrap();

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!(error);
    //     }
    // };

    let f = File::open("hello.txt").expect("파일을 열 수 없습니다.");
}

use std::io;
use std::io::Read;
fn propagating_example() {
    fn read_username_from_file() -> Result<String, io::Error> {
        let f = File::open("hello.txt");
        let mut f = match f {
            Ok(file) => file,
            Err(e) => return Err(e),
        };
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    read_username_from_file().expect("에러!!!!!!!");
}

use std::fs;
fn propagating_with_questionmark_example() {
    // fn read_username_from_file() -> Result<String, io::Error> {
    //     // File::open의 Ok enum값 혹은 Err enum값을 넘긴다.
    //     // 그래서 Err일 때에는 마치 return Err를 한 것처럼 에러가 전파된다.
    //     let mut f = File::open("hello.txt")?;
    //     let mut s = String::new();
    //     f.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    // read_username_from_file().expect("에러~~");

    // fn simple_read_username_from_file() -> Result<String, io::Error> {
    //     let mut s = String::new();
    //     File::open("hello.txt")?.read_to_string(&mut s)?;
    //     Ok(s)
    // }
    // simple_read_username_from_file().expect("에러??");


    fn most_simple_read_username_from_file() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    most_simple_read_username_from_file().expect("에러ㅜㅜ");
}

pub fn run() {
    // result_example();
    // match_example();
    // unwrap_example();
    // propagating_example();
    propagating_with_questionmark_example();
}
