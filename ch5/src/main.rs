#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
fn main() {
	let rect1 = Rectangle { height: 50, width: 30 };
	println!("rect1 is {:?}", rect1);
}
