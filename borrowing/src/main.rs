fn main() {
    let mut s = String::from("Bharath");
    // /creating a reference is called borrowing 
    let length = calculate_length(&s);
    println!("The length of string {s} is {length}");
    change(&mut s);
    println!("The string after mutating : {s}");
    borrow();
}

fn borrow() {
    let mut s : String = String::from("Gurumurthy");
    let s1 = &s;
    let s2 = &s;
    println!("{s1} , {s2} ");
    let s3 = &mut s;
    println!("{s3}");
}
println!("{s}");

fn calculate_length(s : &String) -> usize {
    s.len()
}

fn change(name : &mut String) {
    name.push_str(" B");
}