use rand::Rng;

fn twelve_days_of_christmas() {
    let days = [
        "first",
        "second",
        "third",
        "fourth",
        "fith",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"
    ];

    let all_gifts = [
        "A partridge in a pear tree",
        "And a partridge in a pear tree",
        "Two turtle doves",
        "Three French hens",
        "four calling birds",
        "five gold rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming"
    ];

    println!("On the {} day of Christmas my true love sent to me", days[0]);
    println!("{}", all_gifts[0]);
    for day in 1..12 {
        println!("On the {} day of Christmas my true love sent to me", days[day]);
        // use of the iter to create ranges, this should avoid reallocating a new array 
        // and point to the memory addresses for the all_gifts array
        let gifts = all_gifts.iter().skip(1).take(day + 1).rev(); 
        for gift in gifts {
           println!("{gift}");
        }
    }
}

fn nth_fibonacci(n: i32) -> i32 {
    if n <= 1 {
        n
    } else {
        nth_fibonacci(n - 1) + nth_fibonacci(n - 2)
    }
}

fn fahrenheit_to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn clamp(min: i32, max: i32, val: i32) -> i32 {
    if min > val {
        min
    } else if max < val{
        max
    } else {
        val
    }
}

fn main() {
    println!("{}\n", fahrenheit_to_celsius(145.0));
    println!("{}\n", nth_fibonacci(5));
    twelve_days_of_christmas();

    // Some garbage code with some loop features
    let (max, min) = (100, 1);
    let mut generator = rand::thread_rng();
    let result = 'random_loop: loop {
        let mut counter = 0;
        let counter = loop {
            let random_value =  generator.gen_range(min..=max);
            if random_value == max {
                println!("Max reached");
                break 'random_loop counter 
            }
            counter += 1;
            if counter == max {
                break counter
            }
        };
        if counter == 4 {
            break clamp(20, 50, counter) // semicolon can be added or removed from this expression - no effect
        };
    }; // need to end with semicolon to turn loop expressio into statement

    println!("Looped {result} times");

    let a = [10, 20, 30, 40, 50];
    for element in a { // for loop is great for looping over a collection of elements
        println!("the value is: {element}");
    }
    let mut number = 3;
    while number != 0 { // conditional looping
        println!("{number}!");

        number -= 1;
    }
    println!("LIFTOFF!!!");
    // previous loop as a for loop
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");

}
