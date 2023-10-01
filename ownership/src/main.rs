use std::io;

fn main() {
    let i = "hello"; // fast and efficient, immutable, on stack
    let mut s = String::from("betterString"); //useful, on heap
    let mut s1 = String::from("betterString"); //useful, on heap
    s.push_str(", world!");
    println!("{}", s);
    let (d, s2) = returnsSomething(s, s1); // s, s2 doesn't exist
    let c = doesntTakeOwnership(&d); 
    let mut s3 = String::from("abc");
    canModify(&mut s3);
} // i, c, s2, s3 no longer exists

fn returnsSomething(a: String, i: String) -> (String, String) {
    (a, i) // returns (a, i)
}
fn doesntTakeOwnership(s: &String) -> &String{ // two different pointers exist for s
    // not allowed to modify s, unless s is passed as mutable like this:
    // s: &mut String, also declared mutable in caller
    s
}

fn canModify(s: &mut String) -> &String{
    s.push_str("excess");
    s
}

// let x = 5;
// let y = x;
// x and y two separate integers on stack

//let s1 = String::from("abc");
//let s2 = s1; no separation, s2 and s1 refer to one location on heap
    // CANNOT USE S1, only s2 exists, called a "move"

// have heap var, and call into function parameter, funcion called takes ownership of pointer
