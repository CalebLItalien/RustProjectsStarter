fn main() {
    let s = String::from("hello world");
    let hello = &s[0..5];
    // or let hello = &s[..5];
    let world = &s[6..11];
    //whole word would be let slice = &s[..];
    let mut d = String::from("two words");
    let slicedWord = firstWord(&d);
}
fn firstWord(s: &String) -> &str { // &str is immutable
    let bytes = s.as_bytes(); // so we can iterate

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]; //non inclusive of i
        }
    }
    &s[..]
}