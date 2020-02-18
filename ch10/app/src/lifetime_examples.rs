#![allow(dead_code, unused_variables)]
// fn longest (x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn try_longest () {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);
}
fn try_with_diff_lifetime () {
    let string1 = String::from("아주 아주 긴 문자열");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    // println!("더 긴 문자열: {}", result);
}




pub fn run () {
    // try_longest();
    try_with_diff_lifetime();
}