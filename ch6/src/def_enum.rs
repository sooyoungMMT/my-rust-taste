// #[allow(dead_code, unused_variables)]
#![allow(warnings)]

#[derive(Debug)]
enum IpAddrKind {
    V4, V6
}
#[derive(Debug)]
enum IpAddrKindWithValue {
    V4(String),
    V6(String)
}
#[derive(Debug)]
enum IpAddrWithRange {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum Message {
    Quit, 
    Move {x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32)
}
impl Message {
    fn call (&self) {
        println!("{:?}", self); // Write("hello rust!")
    }
}



fn the_value_of_enum () {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("four: {:?}", four); // four: V4
    println!("six: {:?}", six);  // six: V6

    let home = IpAddrKindWithValue::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKindWithValue::V6(String::from("::1"));
    println!("home: {:?}", home); // home: V4("127.0.0.1")
    println!("loopback: {:?}", loopback); // loopback: V6("::1")

    let home2 = IpAddrWithRange::V4(127,0,0,1);
    let loopback2 = IpAddrWithRange::V6(String::from("::1"));
    println!("home2: {:?}", home2); // home2: V4(127, 0, 0, 1)
    println!("loopback2: {:?}", loopback2);  // loopback2: V6("::1")
}
fn message_example () {
    let m = Message::Write(String::from("hello rust!"));
    m.call();
}

pub fn run() {
    the_value_of_enum();
    message_example();
}
