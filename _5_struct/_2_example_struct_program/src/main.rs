#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: (30),
        height: (50),
    };
    println!("The area of the rectangle {:?} is {}", rect1, area(&rect1));
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
