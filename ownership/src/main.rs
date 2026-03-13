fn takes_ownership(some_string: String) {
    println!("{some_string}");
}

fn gives_ownership() -> String {
    let some_string = String::from("A gift!");
    some_string
}

fn takes_and_restores_ownership(some_string: String) -> String {
    some_string
}

fn makes_copy(some_int: i32) {
    println!("{some_int}");
}

fn get_length(string: &String) -> usize {
    string.len()
}

fn change_borrowed(string: &mut String) {
    string.push_str(", World!");
}

// This function will create a dangling pointer as when it returns,
// the variable 's' will go out of scope and the heap data will be dropped.
// Thus we are attempting to return a reference to data that is no longer
// valid, which is prevented by the compiler. 
// 
// fn dangle() -> &String {
//     let s = String::from("Hello World!");
//     &s
// }
// 
// Instead, we can transfer the ownership of the variable to the caller by 
// returning it

fn no_dangle() -> String {
    let s = String::from("Hello World!");
    s
}

fn main() {
    println!("Ownership!");

    // stack variable (implements the Copy trait)
    {
        // This is a fixed size string literal
        let greeting = "Hello!";
        println!("{greeting}"); // will cause error
                                // that will go out of scope and cause a compiler error
                                // if trying to access when out of scope
    }
    // println!("{greeting}"); // will cause error

    // heap variable (implements the Drop trait)
    {
        let mut greeting = String::from("Hello"); // must be mutable
        greeting.push_str(", World!"); // to let us modify the string
        println!("{greeting}");
    }

    // moving heap variables
    {
        let hello = String::from("Hello");
        let moved_hello = hello;
        // println!("{hello}"); // no longer valid - compile error
        println!("{moved_hello}");
    }

    // copying heap variables
    {
        let hello = String::from("Hello");
        let cloned_hello = hello.clone(); // creates a deep copy
        println!("{hello} {cloned_hello}!");
    }

    // copying and moving variables
    {
        let string = String::from("Hello World!");
        takes_ownership(string);
        // string.bytes(); // string was moved, no longer valid
        let x = 5;
        makes_copy(x);
        println!("{x}") // integer was copied, still valid
    }

    // ownership of return values
    {
        let s1 = gives_ownership();
        let s2 = s1;
        let s3 = takes_and_restores_ownership(s2);
        takes_ownership(s3); // prints "A gift!"
    }

    // references
    {
        let greeting = String::from("Hello!");
        // passing a reference means passing a pointer to data instead of transferring ownership 
        let length = get_length(&greeting);
        println!("Length is {length}");
        println!("String was \"{greeting}\"");
    }

    // mutable references
    {
        let mut greeting = String::from("Hello");
        change_borrowed(&mut greeting);
        println!("{greeting}");
        // can also make an immutable ref to a mutable variable
        let length = get_length(&greeting);
        println!("Length is {length}");
    }
}
