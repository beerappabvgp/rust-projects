use std::fmt::Display;

fn main() {
    println!("Hello world!!");
    another_function(5);
    // statement vs expression 

    let y = {
        let x = 4;
        x + 4
    };
    println!("{:?}" , y);
    println!("five function value is : {:?}" , five());
    println!("The plus one value is : {} " , plus_one(10));
}

// statement -> I will do all the things specified but i dont get resolved to a value at end , i only perform actions or side effects but dont resolve to a value
// expression -> expression will always resolve to a value at the end for example function call , literal values etc..

fn five() -> i32 {
    5 // this is the expression which returns value
}

fn plus_one(x : i32) -> i32 {
    x + 1 // This is an expression which returns the value 
}

fn another_function(x : i32) {
    println!("The value is : {} " , x);
}

