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

    println!("Display test: {}", tweet);

    let test_vec = vec![1,2,3,4,5];
    println!("test_vec impl Summary: {}", test_vec.summarize());


    // ==========

    let article = aggregator::NewsArticle {
        headline: String::from("대한민국, 러시아 월드컵 예선에서 독일을 이겼다"),
        location: String::from("카잔 아레나, 러시아"),
        author: String::from("위키백과"),
        content: String::from("2019년 6월 27일...")
    };
    println!("새로운 기사: {}", article.summarize());
}