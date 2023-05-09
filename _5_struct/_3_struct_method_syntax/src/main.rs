#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        return (self.width > other.width) && (self.height > other.height);
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1: Rectangle = Rectangle {
        width: (30),
        height: (50),
    };

    let rect2: Rectangle = Rectangle {
        width: (10),
        height: (40),
    };

    let rect3: Rectangle = Rectangle {
        width: (60),
        height: (45),
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect3));

    let sq: Rectangle = Rectangle::square(2);

    println!("Square: {:?}", sq);
}
