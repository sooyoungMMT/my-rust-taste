mod def_enum;
mod option_enum;
mod match_example;

fn main() {
    // ====================
    // def_enum::run();
    // option_enum::run();
    match_example::run();
    // ====================
    add_margin();
}


fn add_margin() {
    for _x in 0..5 {
        println!("");
    }
}