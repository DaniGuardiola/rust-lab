#[derive(Debug)] // makes struct inherit debug formatting functionality
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

// can be separated into multiple impl blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

fn main() {
    let rect = Rectangle {
        height: 30,
        width: 50,
    };
    println!("{{:?}}  - rect is {:?}", rect); // one line
    println!("{{:#?}} - rect is {:#?}", rect); // multiple lines

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("Square of 28: {:#?}", Rectangle::square(28));
    println!("Square of 65: {:#?}", Rectangle::square(28));
}
