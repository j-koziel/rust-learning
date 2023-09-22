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
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    // println!("rect1 is {:#?}", rect1);
    // dbg!(&rect1);

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2))
}

fn area(rectangle: &Rectangle) -> u32 {
    return rectangle.width * rectangle.height;
}
