use std::collections::HashMap;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let x = vec![1, 2, 3];
    for i in &x {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }
    let third: &i32 = &v[2]; // panicks if invalid

    let third2: Option<&i32> = v.get(2); // returns None if invalid

    let row = vec![
        MultiTypes::Int(3),
        MultiTypes::Text(String::from("blue")),
        MultiTypes::Float(10.12),
    ];

    let data = "initial contents";
    let s = data.to_string();
    let d = "initial_contents".to_string();
    let mut hello = String::from("hola");
    hello.push_str("bar");
    println!("{}", hello);
    let s3 = s + &hello;

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");

    for c in "abc".chars() {
        println!("{c}");
    }
    for b in "abc".bytes(){
        println!("{b}");
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0); // sets scores to 0 if nonexistant key

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
    let mut scores2 = HashMap::new();
    scores2.insert(String::from("Blue"), 10);
    scores2.insert(String::from("Blue"), 25);

    scores2.entry(String::from("Yellow")).or_insert(50);
    println!("{:?}", scores2);

}  

enum MultiTypes {
    Int(i32),
    Float(f64),
    Text(String),
}