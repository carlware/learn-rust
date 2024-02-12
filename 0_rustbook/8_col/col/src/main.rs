use std::collections::HashMap;

fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn main6() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    println!("{field_name}");
    println!("{field_value}");
}

fn main5() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("{s}");
    // println!("{s1}"); # don't work take ownership of s1
}

fn main4() {
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("{s}");
    println!("{s1}");
}

fn main3() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;
    // println!("{s1}"); #dont work because was moved
    println!("{s3}");
}

fn main2() {
    let mut s1 = String::from("foo");
    let s2 = "bar";
    println!("s2 is {s2}");
    s1.push_str(s2);
}

fn main1() {
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
