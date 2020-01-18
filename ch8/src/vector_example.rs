pub fn new_vector () {
    println!("new vector!");

    
    let v: Vec<i32> = Vec::new();
    let v2 = vec![1,2,3];   
    println!("new vector: {:?}, {:?}", v, v2);
}


pub fn update_vector() {
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    println!("update vector: {:?}", v);
}

pub fn read_value_in_vector() {
    let v = vec![1,2,3,4,5];

    let third: &i32 = &v[2];
    println!("세번째 원소: {}", third);

    match v.get(2) { 
        Some(third) => println!("세번째 원소: {}", third),
        None => println!("세번째 원소가 없습니다.")
    }

    // let seventh: &i32 = &v[6]; // panic 발생
    // match v.get(7) { // get 메서드는 None으로 우아하게 처리한다.
    //     Some(seventh) => println!("7번째 원소: {}", seventh),
    //     None => println!("7번째 원소가 없습니다.")
    // }
}