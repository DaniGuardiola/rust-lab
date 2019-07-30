#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    // string slices
    // -------------
    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    // String slices contain a pointer to the start and the length
    // of a substring of a String value stored in the heap

    let s = String::from("hello");

    let slice = &s[0..2];
    let slice = &s[..2]; // 0 can be omitted

    let len = s.len();

    let slice = &s[3..len];
    let slice = &s[3..]; // len can be omitted

    let slice = &s[0..len];
    let slice = &s[..]; // both can be omitted to take a slice
                        // of the entire String

    // string literals are slices (&str) that point to the
    // string data in the binary file

    let my_string = String::from("hello world");

    // first_word works on slices of `String`s
    let word = first_word_slice(&my_string[..]);

    let my_string_literal = "hello world";

    // first_word works on slices of string literals
    let word = first_word_slice(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word_slice(my_string_literal);

    // other slices
    // -------------

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3]; // slice of type &[i32]
}

// the type that signifies “string slice” is written as &str
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// better to take a slice as parameter
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
