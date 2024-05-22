struct Rectangle {
    width : u32,
    height : u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self , rect : &Rectangle) -> bool {
        self.width > rect.width && self.height > rect.height
    }

    fn square(size : u32) -> Self {
        Self {
            width : size,
            height : size,
        }
    }

}


fn main() {
    let rect1 = Rectangle {
        width : 100,
        height : 100,
    };
    let rect2 = Rectangle {
        width : 200,
        height : 200,
    };
    let rect3 = Rectangle {
        width : 50,
        height : 50,
    };
    println!("The area of rectangle is : {}" , rect1.area());
    println!("Can rect1 hold rect2 ? : {}" , rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3 ? : {}" , rect1.can_hold(&rect3));
    let square = Rectangle::square(4);
    
}