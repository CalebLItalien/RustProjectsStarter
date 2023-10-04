fn main() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("Got: {}", val);
    // }
    // assert_eq!(v1_iter.next(), Some(&1));
    // assert_eq!(v1_iter.next(), Some(&2));
    // assert_eq!(v1_iter.next(), Some(&3));
    // assert_eq!(v1_iter.next(), None);

    let total: i32 = v1_iter.sum();
    assert_eq!(total, 6);

    let v2: Vec<i32> = vec![1, 2, 3];
    let v3: Vec<_> = v2.iter().map(|x| x + 1).collect();
}
