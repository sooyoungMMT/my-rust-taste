#![allow(warnings)]

fn add_some_to_number () {
    let some_number = Some(5);
    let number = 5;

    // 아래의 두 식은 컴파일 할 수 없다.
    let sum1 = some_number + some_number; 
    let sum2 = some_number + number;
}


pub fn run() {
    add_some_to_number();
}