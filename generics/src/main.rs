fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point{x: 5, y: 4.0};
    let integer_and_point = MultipleTypePoints {x: 5, y: 4.0}; 
    println!("integer.x = {}", integer.x());
}

// fn largest<T>(list: &[T]) -> &T {
    
// }

struct Point<T> {
    x: T,
    y: T,
}

struct MultipleTypePoints<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> { // could be impl Point<f32>, for specific type
    fn x(&self) -> &T {
        &self.x
    }
}