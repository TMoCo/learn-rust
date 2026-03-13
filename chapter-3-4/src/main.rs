fn naive_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn main() {
    let string = String::from("Hello World");
    let word_length = naive_first_word(&string);
    println!("The length of the first word is: {word_length}");
    let hello = &string[..word_length]; // slice = a range
    // range indices cannot be in middle of multibyte characters
    // omit first index will default to 0, omit last index will default to end of enurerable
    let world = &string[word_length + 1..];
    println!("{hello} {world}");
    
    // `first_word` also works on references to `String`s, which are equivalent to whole slices of `String`s.
    let word = first_word(&string);
    println!("The first word is: \"{word}\"");

    let string_literal = "Hello world";

    // `first_word` works on slices of string literals, whether partial or whole.
    // let word = first_word(&string_literal[..word_length]);
    // let word = first_word(&string_literal[..]);

    // Because string literals *are* string slices already, this works too, without the slice syntax!
    let word = first_word(string_literal);
    println!("The first word is: \"{word}\"");

    {
        // slicing collections other than strings, eg arrays
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }
}
