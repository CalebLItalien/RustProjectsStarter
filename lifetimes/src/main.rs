use std::fmt::Display;

fn main() {
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // }
    // println!("r: {}", r); // throws error, r out of scope

    let x = 5;
    let r = &x;
    println!("r: {}", r);

    let string1 = String::from("abcd");
    let string2 = "xyz";
    let result = longest(string1.as_str(), string2);
    println!("longer: {}", result);

    let s: &'static str = "I have a static lifetime."; // always available
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    }
    else{
        y
    }
}
// &i32 === a reference
// &'a i32 === a reference with an explicit lifetime
// &'a mut i32 === a mutable reference with an explicit lifetime

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}