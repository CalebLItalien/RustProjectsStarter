use std::fs::File;
use std::fs;
use std::io::{self, Read, ErrorKind};

fn main() {
    // let v = vec![1, 2, 3];
    // v[99];
    // panic!("crash");
    let greeting_file = File::open("hello.txt");
    let greeting_file_e = match greeting_file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
    let greeting = File::open("blank.txt").unwrap(); // calls panic for you
    let greeting = File::open("blank.txt")
        .expect("blank.txt should be included");
}
// to get stacktrace, run as
//RUST_BACKTRACE=1 cargo run

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username_file = File::open("helloll.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
    // ? is syntactic sugar, returns errors if they happen
    // ? ONLY for functions that return
}

fn read_file_easy() -> Result<String, io::Error> {
    fs::read_to_string("helleads.txt")
}