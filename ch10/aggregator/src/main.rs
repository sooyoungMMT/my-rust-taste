extern crate aggregator;
use aggregator::Summary; // 이걸 추가해야 tweet.summarize를 쓸 수 있다.


fn main() {
    let tweet = aggregator::Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("just started studying Rust language"),
        reply: false,
        retweet: false
    };
    println!("새 트윗 1개: {}", tweet.summarize());

    println!("std::fmt::Display test: {}", tweet);
}