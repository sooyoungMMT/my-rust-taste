use std::io;


fn counting_chars (my_str: &String) -> i32 {
    let mut count = 0;
    loop {
        match my_str.pop() {
            Some(c) => {
                count = count+1;
            },
            None => {
                break;
            }
        }
    }
    count
}


fn main () {
    println!("Enter any string: ");
    let mut my_string = String::new();
    match io::stdin().read_line(&mut my_string) {
        Ok(n) => {
            let length = counting_chars(&my_string);
            println!("length: {}", length);
            println!("{} bytes read", n);
            println!("{}", &my_string);
        }
        Err(error) => println!("error: {}", error),
    }
}

