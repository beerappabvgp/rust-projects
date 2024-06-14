#[derive(Debug)]
// struct Point<T> {
//     x : T,
//     y : T,
// }

// fn main() {
//     let integer = Point { x: 10 , y : 20 };
//     let float = Point { x : 4.5 , y : 7.8 };
//     println!("{:?}" , integer);
// }

struct Point<T> {
    x: T,
    y: T,
}


enum Option<T> {
    Some(T),
    None,
}


enum Result<T,E> {
    Ok(T),
    Err(E),
}


impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}


fn main() {
    let both_integer = Point { x : 89 , y : 90};
    let integer_float = Point { x: 56 , y: 89 };
    let both_float = Point { x : 1.0 , y : 2.0};
    println!("{:?}" , integer_float);
    println!("{:?}" , both_integer.x());
    println!("{:?}" , both_float.distance_from_origin());
}