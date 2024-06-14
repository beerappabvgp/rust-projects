pub trait Summary {
    fn summarize(&self) -> String {
        String::from("Read more...")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})" , self.headline , self.author , self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}" , self.username , self.content)
    }
}

pub fn notify(item : &impl Summary) {
    println!("Breaking news ! {:?} " , item.summarize());
}


fn main() {
    let tweet = Tweet {
        username: String::from("Bharath"),
        content: String::from("of course , as you probably know me."),
        reply: false,
        retweet: false
    };
    println!("One new tweet : {}" , tweet.summarize());
    let newsletter = NewsArticle {
        headline: String::from("Hello"),
        location: String::from("palamaner"),
        author: String::from("Bharath"),
        content: String::from("This is the news letter that has garned highest views in history"),
    };
    println!("News letter news : {:?}", notify(&newsletter));
    println!("News letter news : {:?}", newsletter.summarize());
    println!("Longest is : {}" , longest(String::from("name").as_str(), String::from("Bharath").as_str()));
}

fn longest<'a>(x: &'a str , y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

