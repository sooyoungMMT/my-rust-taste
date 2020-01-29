#![allow(dead_code, unused_variables)]
fn new_vector () {
    println!("new vector!");

    let v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];   
    println!("new vector: {:?}, {:?}", v, v2);
}
fn update_vector() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("update vector: {:?}", v);
}
fn access_item_in_vector() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("세번째 원소: {}", third);

    match v.get(2) { 
        Some(third) => println!("세번째 원소: {}", third),
        None => println!("세번째 원소가 없습니다.")
    }

    // let seventh: &i32 = &v[6]; // panic 발생 -> index out of bound
    let _seventh = v.get(7);
    match v.get(7) { // get 메서드는 None으로 우아하게 처리한다.
        Some(_seventh) => println!("7번째 원소: {}", _seventh),
        None => println!("7번째 원소가 없습니다.")
    }
    // if let 버전
    if let Some(_seventh) = v.get(7) {
        println!("if let - 7번째 원소: {}", _seventh);
    } else {
        println!("if let - 7번째 원소가 없습니다/");
    }
}
fn access_item_while_holding_ref () {
    let mut v = vec![1,2,3,4,5];
    let first = &v[0];
    v.push(6);
    let r1 = &mut v;

    // println!("{}", first); // print를 지우면 rust의 최적화 작업 때문에 에러 안난다.
}
fn iterating_over_values_in_vector () {
    // original code
    // let v = vec![2,4,6,8,10];
    // for i in &v {
    //     println!("{}", i);
    // }

    // why?????????????????
    // let mut v = vec![2,4,6,8,10];
    // for i in &v {
    //     println!("{}", i);
    // }
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }
    // println!("{}", &v[0]);


    // error
    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &mut s;
    // println!("{}", r1);
}

fn using_enum_to_store_multiple_types () {
    enum SpreadsheetCell {
        Int(i32), 
        Float(f64),
        Text(String)
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue"))
    ];
}


pub fn run () {
    // new_vector();
    // update_vector();
    // access_item_in_vector();
    // access_item_while_holding_ref();
    // iterating_over_values_in_vector();
    using_enum_to_store_multiple_types();
}