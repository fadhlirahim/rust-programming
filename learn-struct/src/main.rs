#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl is implemenation block
impl Rectangle {
    // know &self
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // method with self and more params
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // associated function does not have self
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rec = Rectangle { width: 30, height: 50 };
        
    println!("rect1 is {:?}", rec);
    println!("rec.width = {}", rec.width);
    println!("rec.height = {}", rec.height);
    println!("area = {}", Rectangle::area(&rec));

    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(3);

    println!("Square is {:?}", square);
}

