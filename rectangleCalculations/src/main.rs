fn main() {
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );
    let rect1 = (30, 50);
    println!(
        "The area of the second rectangle is {} square pixels.",
        areaTuple(rect1)
    );
    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect4 = Rectangle {
        width: 50,
        height: 400,
    };
    println!(
        "The area of the third rectangle is {} square pixels.",
        areaStruct(&rect3)
    );
    println!("rect3 is {:?}", rect3); // can also use {:#?}
    dbg!(&rect3);
    println!("The area of rect3 is {}",
        rect3.area());
    println!("rect4 hold rect3? {}", rect4.can_hold(&rect3));
    let sq = Rectangle::square(3);
}
fn area(width: u32, height:u32) -> u32 {
    width * height
}
fn areaTuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
fn areaStruct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Self { // returns new rectangle object
        Self{
            width: size,
            height: size,
        }
    }
}
