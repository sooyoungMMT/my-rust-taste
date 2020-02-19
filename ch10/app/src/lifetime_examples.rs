#![allow(dead_code, unused_variables)]
// fn longest (x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn try_longest() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("더 긴 문자열: {}", result);
}
fn try_with_diff_lifetime() {
    let string1 = String::from("아주 아주 긴 문자열");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("더 긴 문자열: {}", result);
    }
    // println!("더 긴 문자열: {}", result);
}

// fn longest_wrong_lifetime<'a> (x: &str, y: &str) -> &'a str {
//     let result = String::from("아주 아주 긴 문자열");
//     result.as_str()
// }

fn struct_with_lifetime() {
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }
    let novel = String::from("스타워즈. 오래 전 멀고 먼 은하계에...");
    let first_sentence = novel
        .split('.')
        .next()
        .expect("문장에서 마침표 '.'를 찾을 수 없습니다.");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("i: {}", i.part);
}


fn lifetime_in_method () {
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("주목해주세요! :{}", announcement);
        self.part
    }
}
}


pub fn run() {
    // try_longest();
    // try_with_diff_lifetime();
    // longest_wrong_lifetime("1234", "3456");
    struct_with_lifetime();
}
