pub mod front_of_house;

pub use front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    let breakfast = back_of_house::Breakfast::summer(String::from("wheat"));
    println!("The bread is : {}" , breakfast.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // breakfast.seasonal_fruit = String::from("Black berries");

    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;

}


mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: String) -> Breakfast {
            Breakfast {
                toast: toast,
                seasonal_fruit: String::from("Berries"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,  
    }

}