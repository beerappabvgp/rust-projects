fn main() {
    let s : String = String::from("Hello ,world!!");
    let hello : &str = &s[..5];
    let world : &str = &s[6..];
    println!("{hello} , {world}");
    let first_word = first_word(&s);
    println!("{first_word}");

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice , &[2,3]);
}

fn first_word(s : &String) -> &str {
    let bytes = s.as_bytes();
    for (i , &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}