use std::fmt::Display;

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("주목하세요! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn try_example(){
    let string1 = String::from("아주아주아주아주 긴 텍스트");
    let string2 = String::from("짧은 텍스트");
    let result = longest_with_an_announcement(&string1, &string2, "attention");
    println!("결과: {}", result);
}


pub fn run() {
    try_example();
}
