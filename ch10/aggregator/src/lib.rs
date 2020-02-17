use std::fmt::Display;
use std::fmt::Result;

pub trait Summary {
    // fn summarize(&self) ->  String;
    fn summarize(&self) -> String {
        String::from("(계속 읽기)")
    }
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
impl Summary for Tweet {
    fn summarize(&self) -> String {      
        format!("{}: {}", self.username, self.content)
    }
}
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        write!(f, "({}, {})", self.username, self.content)
    }
}


impl Summary for Vec<i32> {
    fn summarize(&self) -> String {      
        format!("vector summary: {}", self.len())
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
