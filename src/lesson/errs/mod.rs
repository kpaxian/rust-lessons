use std::fs::File;
use std::io::{ErrorKind, Read};

pub fn init() {
    read_username_from_file();
    let username = read_username_from_file();

    println!("Username is {:?}", username);
    //foo();
    //file_open_handler();
}

fn read_username_from_file() -> Result<String, std::io::Error> {
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }





}

fn file_open_handler() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => {
            println!("file is: {:?}", file);
            file
        },
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e)
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        }
    };
}

fn foo() {
    let vec = vec![1, 2, 3];
    println!("{:?}", vec[2]);
}