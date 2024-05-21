fn main() {
    // ownership 

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("The s1 string is : {} , {} " , s1 , s2);

    takes_ownership(s1);
    let x : i32 = 90;
    makes_copy(x);
    let s3 = gives_ownership();
    let s4 = String::from("Anil");
    let s5 = takes_and_gives_back_ownership(s4);
    println!("{s5} , {s3}")
}

fn takes_and_gives_back_ownership(s : String) -> String {
    s
}

fn gives_ownership() -> String {
    let s = "Bharath".to_string();
    return s;
}

fn takes_ownership(s : String) {
    println!("{}" , s);
}
// here s goes out of scope and drop is called 

fn makes_copy(x : i32) {
    println!("{x}");
}
