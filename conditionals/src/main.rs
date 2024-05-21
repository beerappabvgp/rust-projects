use core::num;

fn main() {
    let number : i32 = 100;
    if number == 100 {
        println!("condition was true");
    } else {
        println!("The condition was false");
    }

    if_is_a_expression();
    if_evaluates_to_boolean();
    println!("The day is : {}" , print_days(1))
}

// If is not a statement in rust it is an expression 

fn if_is_a_expression() {
    let number = 3;
    let result = if number == 3 {
        "number is equal"
    } else {
        "number is either greater or lesser"
    };
    println!("{}" , result);
}

fn if_evaluates_to_boolean() {
    let number = 90;
    if number != 0 {
        println!("The {} is grater than 0" , {number});
    }
}


// multiple conditions with else if 

fn print_days(day : i32) -> String {
    return if day == 1 {
        "Monday".to_string()
    } else if day == 2 {
        "Tuesday".to_string()
    } else if day == 3 {
        "Wednesday".to_string()
    } else {
        "Some other day".to_string()
    }
}