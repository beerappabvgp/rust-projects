use std::{ffi::c_long, fmt::Display};
use std::io;


fn main() {
    let age : i32 = 19;
    let salary : f64 = 88888888.8888;
    let is_satisfied : bool = true;
    const INITIAL : char = 'B';
    let tup : (i32 , f64 , bool , char) = (age , salary , is_satisfied , INITIAL);
    println!("{:?}", tup);
    println!("{}" , age);
    let numbers : (i32 , f64 , i32) = (1,2.3,4);
    let (x,y,z) = numbers;
    println!("{} , {} , {}" , x , y , z);
    println!("{}" , numbers.0);
    arrays();
    index_out_of_bounds();
}


// The array type 

fn arrays() {
    let a : [i32 ; 4] = [1,2,3,4];
    let b : [i32 ; 5] = [5 ; 5];
    println!("{:?}" , a);
    println!("{:?}" , b);
    let first : i32 = a[0];
    println!("{}" , first);
}

fn index_out_of_bounds() {
    let a : [i32 ; 5] = [5 ; 5];
    let mut index = String::new();
    io::stdin()
    .read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Please enter the number");
    let element : i32 = a[index];
    println!("The value of the element at index {} is  {} " , {index} , {element});
}





