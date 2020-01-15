extern crate simple_use;

use simple_use::a::series::of;
use simple_use::a::series::of::nested_modules;

use simple_use::TrafficLight;
use TrafficLight::{Red, Yellow};

fn main() {
    simple_use::a::series::of::nested_modules();
    of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("red: {:?}, yellow: {:?}, green: {:?}", red, yellow, green);
}
