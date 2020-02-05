#![allow(dead_code, unused_variables)]

fn panic_example () {
    panic!("crush and burn!!");
}
fn backtrace_example () {
    let v = vec![1,2,3];
    v[99];
}

pub fn run() {
    // panic_example();
    // backtrace_example();
}