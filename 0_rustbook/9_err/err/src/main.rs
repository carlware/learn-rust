
use std::fs::File;
use std::io::ErrorKind;
use std::io::{self, Read};

fn ex1_not_found() {
    let file_result = File::open("hello.txt");

    let _file = match file_result {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}

fn ex2_handle_diff_errs() {
    let file_result = File::open("hello.txt");

    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_err => {
                panic!("Problem opening the file: {:?}", other_err)
            },
        }
    };
}

fn read_username(path: String) -> Result<String, io::Error> {
    let file_result = File::open(path);

    let mut file = match file_result {
        Ok(file) => file,
        Err(error) => return Err(error),
    };

    let mut username = String::new();
    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(error) => Err(error),
    }
}

fn ex2_handle_with_closure() {
    let _file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

fn read_username_from_file(path: String) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file_shorter(path: String) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(path)?.read_to_string(&mut username)?;
    Ok(username)
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//
//     Ok(())
// }

fn main() {
    // panic!("crash and burn");

    // ex2_handle_with_closure();

    // this will panic
    // let greeting_file = File::open("hello1.txt")
    //     .expect("hello.txt should be included in this project");

    let username = read_username_from_file(String::from("hello.txt")).unwrap_or(String::from("default"));
    println!("username: {:?}", username);
}
