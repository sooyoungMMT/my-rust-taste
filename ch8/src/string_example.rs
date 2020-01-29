#![allow(dead_code, unused_variables)]
fn new_string () {
    let s = String::new();

    let data = "문자열 초깃값";
    let ss = data.to_string();
    println!("{}", ss);

    let s3 = String::from("문자열 초깃값");
}
fn modify_string() {
    let mut s = String::new();
    s.push_str("hello world");
    s.push('!');
    println!("{}", &s);

    let s2 = "bar";
    s.push_str(s2);
    println!("s2: {}", s2);

    let s3 = String::from("Hello ");
    let s4 = String::from("world!");
    let s5 = s3 + &s4;
    println!("s5: {}", s5);
    println!("s4: {}", s4);
    // println!("s3: {}", s3); // error!

    let new_str = format!("{}-{}-{}", s, s2, s4);
    println!("with format! - {}", new_str);
}
fn index_of_string () {
    let hello = "안녕하세요";
    // let answer = &hello[0]; // error!
    let answer = &hello[0..3];
    let answer2 = &hello[0..1]; // 유효하지 않은 값. 다만 런타임에 패닉이 발생한다.
    println!("&hello[0]: {}", answer);
}
fn iterating_over_strings () {
for c in "안녕하세요".chars() {
    println!("{}", c);
}
for b in "안녕하세요".bytes() {
    println!("{}", b);
}
}


pub fn run () {
    // new_string();
    // modify_string();
    // index_of_string();
    iterating_over_strings();
}