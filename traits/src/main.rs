//use aggregator::{Summary, NewsArticle};

fn main() {
    let newsArticle = NewsArticle {
        headline: String::from("user"),
        content: String::from(
            "abc",
        ),
        location: String::from("abc"),
        author: String::from("abc"),
    };
    println!("1 new article: {}", newsArticle.summarize());
}

pub trait Summary { // basically just an interface
    fn summarize(&self) -> String;
    fn auto_summary(&self) -> String {
        String::from("(Read more...)")
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
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub fn notify(item: &impl Summary) {
    println!("Breaking news {}", item.summarize());
}

pub fn verbose_notify<T: Summary>(item: &T) {
    println!("verbose");
}

//to implement two traits
// pub fn notify(item: &(impl Summary + Display)) {} 

// to implement multiple traits cleanly
// fn some_function<T, U> (t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
//     {}

fn returns_sumarizable() -> impl Summary {
    NewsArticle {
        headline: String::from("user"),
        content: String::from(
            "abc",
        ),
        location: String::from("abc"),
        author: String::from("abc"),
    }
} // can only return one type in this function, 
//no switch cases for diff implementations of summary