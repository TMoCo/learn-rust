#[derive(Debug)] // setting this attribute lets us print the struct using the :? format specifier
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let width1 = 30;
    let height1 = 50;

    {
        // naive, expect dimensions to be passed as separate parameters
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    {
        // better, groups dimensions into a single parameter
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        println!(
            "The area of the rectangle is {} square pixels.",
            area((width1, height1))
        );
    }

    {
        // best, use struct to name data fields
        let rect1 = Rectangle {
            width: width1,
            height: height1,
        };

        // println!("The rectangle is {rect1}"); errror as rect1 does not implement the Display trait
        println!("The rectangle is {rect1:?}"); // compiles provided the struct has the Debug attribute
        println!("The rectangle is {rect1:#?}"); // same as above, but with each data field in a new line
        println!(
            "The area of the rectangle is {} square pixels.",
            area(&rect1)
        );

        dbg!(&rect1); // dbg macro prints to stderr

        let rect2 = Rectangle {
            width: dbg!(rect1.width * 2),
            ..rect1
        };
        dbg!(&rect2);
    }
}
