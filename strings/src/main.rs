fn main() {
    let mut s = String::new();
    // using to_sting
    let s1 = "Initial Data";
    let c = s1.to_string();
    let name : String = String::from("Bharath");
    println!("My name is {name} ");
    // strings are utf encoded
    let hello = String::from("नमस्ते");
    println!("{hello}");
    s.push_str("From push method");
    println!("After appending : {s}");
    s.push('B');
    println!("Pushed character  : {s}");

    let firstName = String::from("Anil");
    let lastName : String = String::from("Beerappa");
    let fullName : String = firstName + &lastName;
    println!("Lastname: {fullName}");
    let tic = String::from("Tic");
    let tac = String::from("Tac");
    let toe: String = String::from("Toe");
    let format = format!("{tic}-{tac}-{toe}");
    println!("{}", format);
    let tic_tac_toe = tic + "-" + &tac + "-" + &toe;
    println!("{tic_tac_toe}");
    for c in "Bharath".chars() {
        println!("char - {c}");
    }
    for c in "Bharath".bytes() {
        println!("Bytes - {c}");
    }
}   