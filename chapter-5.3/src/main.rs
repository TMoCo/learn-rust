#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// implementation bock for Rectangle struct
impl Rectangle {
    // anything inside impl block is associated to the Rectangle type
    fn area(&self) -> u32 {
        // all implementation methods must have &self (short for self: &Self) as their first parameter
        // we borrow the instance we call the method on by passing self as a reference (&self)
        // Self is an alias for the type that the impl block is for
        self.width * self.height
    }

    // if we want to change the instance somehow, we need to borrow the instance with mutable reference, provided the instance iteslf is mutable
    fn double_width(&mut self) {
        self.width *= 2;
    }

    // method with more parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height >= other.height
    }

    // associated function, does not have &self as its first parameter, so it is not a method
    // a method is an associated function that has &self as its first parameter
    fn square(size: u32) -> Self {
        // returns Self - the alia for the type that the impl block is for
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let mut rect2 = Rectangle { ..rect1 };
    rect2.double_width();

    println!(
        "The area of the rectangle is {} square pixels.",
        rect2.area()
    );

    println!("Can rect2 hold rect1? {} ", rect2.can_hold(&rect1));

    let square = Rectangle::square(rect1.width);

    println!("The area of the square is {} square pixels.", square.area());
}
