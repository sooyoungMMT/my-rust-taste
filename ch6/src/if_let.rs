#![allow(warnings)]


fn using_match () {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(3) => println!("three!"),
        _ => ()
    }
}
fn using_if_let () {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }
}

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
fn rewrite_if_let_else() {
    let coin = Coin2::Quarter(UsState::Alabama);
    let mut count = 0;
    match coin {
        Coin2::Quarter(state) => println!("{:?}주의 25센트 동전!", state),
        _ => count += 1
    }


    // count = 0;
    // if let Coin2::Quarter(state) = coin { // state에 copy trait이 없다고 안된다.
    //     println!("{:?}주의 25센트 동전!", state);
    // } else {
    //     count += 1;
    // }
}



pub fn run() {
    // using_match();
    // using_if_let();
    rewrite_if_let_else();
}