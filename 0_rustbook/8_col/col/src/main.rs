
use std::collections::HashMap;

fn main() {
    let v1:Vec<i32> = Vec::new();
    let v2 = vec![1, 2, 3, 5, 6, 7];

    println!("{:?}", v1);
    println!("{:?}", v2);

    let ele = v2.get(10);
    match ele {
        Some(ele) => println!("{:?}", ele),
        None => println!("does not exists")
    }

    for ele in v2 {
        println!("{:?}", ele);
    }

    let mut v3 = vec![1, 2, 3, 4, 5];
    for ele in &mut v3 {
        *ele += 50;
    }
    println!("{:?}", v3);

    let str1 = String::from("hello");
    let str2 = String::from("world");
    let str3 = str1.clone() + &str2;
    let str4 = format!("{}-{}", str1.clone(), str2);
    println!("{:?}", str3);
    println!("{:?}", str4);
    println!("{:?}", str1);

    for c in String::from("नमस्ते").chars() {
        println!("{:?}", c);
    }

    let mut  scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let score = scores.get(&String::from("Blue")).copied().unwrap_or(0);
    println!("score: {:?}", score);

    for (key, value) in scores {
        println!("key: {:?} value: {:?}", key, value);
    }


    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
