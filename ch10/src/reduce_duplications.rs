#![allow(dead_code, unused_variables)]
fn find_biggest_number () {
    let number_list = vec![34, 50, 25, 100, 65];
    let mut largest = number_list[0];

    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    println!("largest: {}", largest);
}

fn get_largest(number_arr: &[i32]) -> i32 {
    let mut largest = number_arr[0];
    // for number in number_arr {
    //     if  *number > largest {
    //         largest = *number;
    //     }
    // }
    
    // number_arr이 컬렉션이라 iter()를 굳이 하지 않아도 for-in문이 작동하는 것 같다..?
    for &number in number_arr.iter() {
        if number > largest {
            largest = number;
        }
    }
    largest
}
fn refactor_find_biggest_number () {
    let mut number_list = vec![34, 50, 25, 100, 65];
    let mut largest = get_largest(&number_list);
    println!("가장 큰 수 1: {}", largest);

    number_list = vec![22, 100, 2, 55, 12, 78, 50, 25, 700, 65];
    largest = get_largest(&number_list);
    println!("가장 큰 수 2: {}", largest);
}





pub fn run() {
    // find_biggest_number();
    refactor_find_biggest_number();
}