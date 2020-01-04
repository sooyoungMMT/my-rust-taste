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

pub fn run() {
    // coin_example();
    usstate_example();
}