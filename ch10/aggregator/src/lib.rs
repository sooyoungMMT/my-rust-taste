use std::fmt::Display;
use std::fmt::Result;

pub trait Summary {
    fn summarize(&self) -> String {
        format!("{}님의 기사 더 읽기", self.summarize_author())
    }
    fn summarize_author(&self) -> String;
}


pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        self.author.to_string()
    }
}


pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool
}
impl Summary for Tweet {
    fn summarize(&self) -> String {      
        format!("{}: {}", self.summarize_author(), self.content)
    }
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
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
    fn summarize_author(&self) -> String {
        String::from("summarize_author() in Vec")
    }
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
