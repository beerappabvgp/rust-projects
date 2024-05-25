enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}




fn main() {
    let mut v : Vec<i32> = Vec::new();
    // let v1 = vec![1,2,4,5];
    v.push(12);
    v.push(100);
    v.push(200);
    let numbers = vec![1,2,3,4,5];
    let third: &i32 = &numbers[2];
    println!("The third element is : {} " , third);
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element .")
    }
    // let does_not_exist = &numbers[100];
    let does_not_exist = v.get(100);
    for i in &numbers {
        println!("{i}");
    }

    let mut lucky_numbers = vec![1,2,3,4,5,6];
    for i in &mut lucky_numbers {
        *i *= 2;
        println!("{i}")
    }

    let row = vec![SpreadSheetCell::Int(3) , SpreadSheetCell::Float(4.56) , SpreadSheetCell::Text(String::from("hello"))];


}


