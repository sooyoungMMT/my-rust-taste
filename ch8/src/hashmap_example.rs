use std::collections::HashMap;

fn create_new_hashmap () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
fn create_new_hashmap_with_tuple () {
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
}
fn hashmap_ownership () {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}
fn accessing_value_in_hashmap () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    for (key, value) in & scores {
        println!("{}: {}", key, value);
    }

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    match scores.get(&team_name) {
        None => println!("none"),
        Some(score) => println!("score: {}", score)
    }
}
fn update_hashmap () {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // overwriting
    scores.insert(String::from("Blue"), 30);
    println!("{:?}", scores);


    // only inserting a value if the key has no value
    scores.entry(String::from("Blue")).or_insert(77);
    println!("{:?}", scores);

    // updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}



pub fn run() {
    // create_new_hashmap();
    // create_new_hashmap_with_tuple();
    // hashmap_ownership();
    // accessing_value_in_hashmap();
    update_hashmap();
}