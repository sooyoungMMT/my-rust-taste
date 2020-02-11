#![allow(dead_code, unused_variables)]

fn largetst_i32 (list: &[i32]) -> i32{
    let mut largest = list[0];
    for &number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

// fn largest<T>(list: &[T]) -> T {
//     let mut largest_val = list[0];
//     for &val in list.iter() {
//         if val > largest_val {
//             largest_val = val;
//         }
//     }
//     largest_val;
// }

fn refactor_with_generic () {
    let number_list = vec![2,3,6,1,23,77,13,103];
    let result = largetst_i32(&number_list);
    println!("largest_i32: {}", result);
    // result = largest(&number_list);
}



fn struct_with_generic() {
    struct Point<T, U> {
        x: T,
        y: U
    }
    let integer_and_float = Point {x: 1, y: 1.0};
    let float_and_float = Point {x: 4.0, y: 1.0};
}

fn using_in_method_declaration () {
    struct Point<T> {
        x: T,
        y: T
    }
    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point {x: 5, y: 10 };
    println!("p.x = {}", p.x());
}

fn diff_generic_type () {
    struct Point<T, U> {
        x: T,
        y: U
    }
    impl<T, U > Point<T, U> {
        fn mixup<V, W> (self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y
            }
        }
        fn assoc() -> Point<i32, i32> {
            Point { x: 1, y: 33 }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    
    println!("p3.x: {}, p3.y: {}", p3.x, p3.y);
}




pub fn run (){
    // refactor_with_generic();
    // struct_with_generic();
    // using_in_method_declaration();
    diff_generic_type();
}