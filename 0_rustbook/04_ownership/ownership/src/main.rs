fn main() {
    let s = String::from("hello");              // s comes into scope

    takes_ownership(s);              // s's value moves into the function...
                                                 // ... and so is no longer valid here

    let x = 5;                              // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                                 // but i32 is Copy, so it's okay to still
                                                 // use x afterward

    let mut s = String::from("hello world");

    let word = first_word(&s);

    println!("the first word is: {}", word);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}