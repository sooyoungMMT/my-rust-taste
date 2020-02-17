extern crate aggregator;
use aggregator::Summary;


fn main() {
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("just started studying Rust language"),
        reply: false,
        retweet: false
    };
    println!("새 트윗 1개: {}", tweet.summarize());
    
}