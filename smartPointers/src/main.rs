use crate::List::{Cons, Nil};
use std::{ops::Deref, mem::drop};

fn main() {
    let b = Box::new(5); // i32 value on heap
    println!("b = {}", b);
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPonter{
        data: String::from("abc"),
    };
    drop(c);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn hello(name: &str) {
    println!("hello, {name}!");
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct CustomSmartPonter {
    data: String,
}

impl Drop for CustomSmartPonter {
    fn drop(&mut self) {
        println!("Dropping pointer w data {}", self.data);
    }
}