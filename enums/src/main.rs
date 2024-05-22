#[derive(Debug)]
enum IpAddrKind {
    V4(u8,u8,u8,u8),
    V6(String),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {
        x : i32,
        y : i32,
    },
    Write(String),
    ChangeColor(i32 , i32 , i32),
}

impl Message {
    fn call(&self) {
        println!("{:?}" , self);
    }
}

fn main() { 

    let four = IpAddrKind::V4(1,1,2,254);
    let six: IpAddrKind = IpAddrKind::V6(String::from("127.0.0.2"));
    println!("{:?}" , four);
    println!("{:?}" , six);
    let message = Message::Write(String::from("Hello from Enum"));
    message.call();
    let c:Coin = Coin::Dime;
    let cents = value_in_cents(c);
    println!("{}" , cents);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("{:?}" , none);
    let config_max = Some(34);
    if let Some(max) = config_max {
        println!("The max set is : {}" , max);
    }

}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin : Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x : Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}

