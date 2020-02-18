use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}
impl<T> Pair<T> {
    fn new (x: T, y: T) -> Self {
        Self {
            x, y
        } 
    }
}
impl <T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("가장 큰 멤버는 x: {}", self.x);
        } else {
            println!("가장 큰 멤버는 y = {}", self.y);
        }
    }
}