use crate::UsState::Alaska;

#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("lucky coin");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}

fn main() {
    let home = IpAddr::V4(String::from("127.0.0.1"));
    let v6 = IpAddr::V6(String::from("127.0.0.1"));

    println!("{:?}", home);
    println!("{:?}", v6);


    let m = Message::Write(String::from("hello"));
    m.call();

    // using option
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    // not compile
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y;

    // match expression
    value_in_cents(Coin::Penny);
    value_in_cents(Coin::Quarter(Alaska));

    // option
    println!("value: {:?}", plus_one(Some(5)));


    // matches are exhaustive
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
        // other => move_player(other),
    }

    // if let
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    let coin = Coin::Penny;

    let mut count = 0;
    match coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }
}
