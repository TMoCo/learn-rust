fn main() {
    let width1 = 30;
    let height1 = 50;

    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }

        println!(
            "The area of the rectangle is {} square pixels.",
            area(width1, height1)
        );
    }

    {
        let dimensions = (width1, height1);

        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }

        println!(
            "The area of the rectangle is {} square pixels.",
            area(dimensions)
        );
    }
}
