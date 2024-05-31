use std::{fs::File, io::ErrorKind};

fn main() {
    let greeting_from_file = File::open("hello.txt");
    let greeting_file = match greeting_from_file {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(er) => panic!("problem creating file!")
            },
            other_error => {
                panic!("Problem opening the file : {:?}", other_error);
            }
        }
    };

    let hello_file = File::open("name.txt").expect("Could not find the file !!!!!");

}