use std::ffi::c_long;

fn main() {
    let mut x = 10;
    println!("x : {}" , x);
    x = 100;
    println!("x is : {}" , x);
    const CONST_VALUE : i32 = 23;
    println!("CONST_VALUE :  {} " , CONST_VALUE);

    // shadowing
    let y : i32 = 2;
    let y: i32 = y + 3;
    println!("y inside outer loop is : {} " , y);
    {
        let y : i32 = y * 2;
        println!("y inside inner loop is : {}" , y);
    }

    let spaces = "  ";
    let spaces = spaces.len();
    println!("len of spaces string is : {} " , spaces);  
}