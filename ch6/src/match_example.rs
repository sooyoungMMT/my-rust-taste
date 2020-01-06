#![allow(warnings)]

enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter
}
fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("행운의 페니!");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25
    }
}
fn coin_example() {
    let p = value_in_cents(Coin::Penny);
    let n = value_in_cents(Coin::Nickle);
    let d = value_in_cents(Coin::Dime);
    let q = value_in_cents(Coin::Quarter);

    println!("penny: {}, nickle: {}, dime: {}, quarter: {}", p, n, d, q);
    // 행운의 페니!
    // penny: 1, nickle: 5, dime: 10, quarter: 25
}


//==========
#[derive(Debug)]
enum UsState {
    Alabama, Alaska //...
}
enum Coin2 {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState)
}
fn value_in_cents2(coin: Coin2) -> u32 {
    match coin {
        Coin2::Penny => 1,
        Coin2::Nickle => 5,
        Coin2::Dime => 10,
        Coin2::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
fn usstate_example() {
    let p = value_in_cents2(Coin2::Penny);
    let n = value_in_cents2(Coin2::Nickle);
    let d = value_in_cents2(Coin2::Dime);
    let q = value_in_cents2(Coin2::Quarter(UsState::Alaska));

    println!("penny: {}, nickle: {}, dime: {}, quarter: {}", p, n, d, q);
    // State quarter from Alaska!
    // penny: 1, nickle: 5, dime: 10, quarter: 25
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // error!
    // match x {
    //     Some(i) => Some(i + 1)
    // }
    Some(4) // for compile :-(
}
fn option_example () {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five: {:?}, six: {:?}, none: {:?}", five, six, none);
    // five: Some(5), six: Some(6), none: None
}

fn placeholder_example () {
    let some_u8_value = 0u8; // unsigned 8bit 0
    match some_u8_value {
        0 => println!("zero"),
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => ()
    }
}


pub fn run() {
    // coin_example();
    // usstate_example();
    // option_example();
    placeholder_example();
}