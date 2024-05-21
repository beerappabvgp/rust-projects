fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2
        }
    };
    println!("The result is : {} " , result);
    while_loop();
    while_looping_over_collection();
    for_looping_over_collection();
    take_off_with_for();
}

fn while_looping_over_collection() {
    let a = [1,2,3,4,5];
    let mut index = 0;
    while index < 5 {
        println!("The value is : {} " , a[index]);
        index += 1;
    }
}


fn while_loop() {
    let mut number : i32 = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("Lift Off .... ")
}

fn for_looping_over_collection() {
    let a = [1,2,3,4,5,6,7];
    for element in a {
        println!("The element is : {element}");
    }
}

fn take_off_with_for() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("Lift Off!!!");
}