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
        if self.width > other.width && self.height > other.height {
            return true;
        }

        return false;
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle::square(3);
    let rect3 = Rectangle::new(60, 45);

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("rect1 is {:?}", rect1);
    println!("rect1 is {:#?}", rect1);
    dbg!(rect1);
}
