#[derive(Debug)]
struct Rectangle {
    width : u32,
    height : u32,
}

fn main() {
    let width1 = 30;
    let height1 = 40;
    let rect5 = Rectangle {
        width : 100,
        height : 100
    };
    println!("The area of the rectangle is : {}" , area(width1 , height1));
    let rect1 = (30,20);
    println!("The area of rectangle is : {} " , tuple_struct(rect1));
    println!("{}" , area_struct(&rect5));
    println!("{:?}" , rect5);
}

fn tuple_struct(dimensions : (u32 , u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area(width : u32 , height : u32) -> u32 {
    width * height
}

fn area_struct(rect : &Rectangle) -> u32 {
    rect.width * rect.height
}
