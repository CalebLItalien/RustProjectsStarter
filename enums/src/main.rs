fn main() {
    let four = IpAddrKind::V4(0, 0, 0, 0);
    let six = IpAddrKind::V6(String::from("456"));
    let m = Message::Write(String::from("hello"));
    m.call();

    let some_num = Some(5);
    let some_char = Some('e');
    let absent_number: MyOption<i32> = MyOption::None;

    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // could add else block here as well
}

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
    V7(Ipv7Addr)
}
struct Ipv7Addr{}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
    }
}

enum MyOption<T> {
    None,
    Some(T),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        },
    }
}

fn plus_one(x: MyOption<i32>) -> MyOption<i32> {
    match x {
        MyOption::None => MyOption::None,
        MyOption::Some(i) => MyOption::Some(i + 1),
    }
}
fn dice_rolling(roll: u8) {
    match roll {
        3 => add_hat(),
        7 => remove_hat(),
        other => move_player(), // could also use _ instead of other
        // _ => () does nothing if neither 3 or 7
    }
    fn add_hat() {}
    fn remove_hat() {}
    fn move_player() {}
}