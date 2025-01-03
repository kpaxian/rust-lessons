
use std::collections::HashMap;

pub fn init() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //println!("{:?}", scores);

    let team_name = String::from("Yellow");
    let score = scores.get(&team_name).copied();

    println!("score is {:?}", &score);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}