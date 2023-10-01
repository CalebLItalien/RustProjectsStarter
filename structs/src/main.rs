fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("abc"),
        email: String::from("abc"),
        sign_in_count: 1,
    };
    user1.email = String::from("abcccc");
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("abcdef"),
        sign_in_count: user1.sign_in_count,
    };
    let user3 = User {
        email: String::from("abdafdsac"),
        ..user2 // gets rest of fields from user2
    };
    let black = Color(0, 0, 0);
}
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username, // same as username: username
        email,
        sign_in_count: 1,
    }
struct Color(i32, i32, i32),
}