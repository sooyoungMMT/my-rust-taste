mod def_enum;
mod option_enum;
mod match_example;
mod if_let;

fn main() {
    // ====================
    // def_enum::run();
    // option_enum::run();
    // match_example::run();
    if_let::run();
    // ====================
    add_margin();
}


fn add_margin() {
    for _x in 0..5 {
        println!("");
    }
}