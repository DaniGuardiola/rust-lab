#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length_returning(s1);

    println!("The length of '{}' is {}.", s2, len);

    // this is tedious, it's better to use references

    // references
    // ----------

    let s1 = String::from("hello");

    let len = calculate_length(&s1); // we pass &String instead
                                     // this is called "referencing"

    println!("The length of '{}' is {}.", s1, len);
    // a reference creates a new pointer in the stack, in this case
    // it will be a pointer to the pointer that the string already allocated
    // in the stack that points to the string data in the heap

    let s = String::from("immutable");

    change(&s);

    // mutable references
    // ------------------

    let mut s = String::from("immutable");

    change_mutable(&mut s);

    // you can have only one mutable reference to a particular piece
    // of data in a particular scope

    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s; - ERROR: cannot borrow `s` as mutable more than once at a time

    // println!("{}, {}", r1, r2); - both mutable references used here, incorrect usage

    // same problem when mixing immutable and mutable references

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem

    // let r3 = &mut s; - ERROR: cannot borrow `s` as mutable because it is
    // also borrowed as immutable

    // println!("{}, {}, and {}", r1, r2, r3); - all references used here, incorrect usage

    // this code will compile because the last usage of the immutable references
    // occurs before the mutable reference is introduced:

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
}

fn calculate_length_returning(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

// s is a reference to a String
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

// having references as function parameters is "borrowing"

fn change(some_string: &String) {
    // some_string.push_str(" String value"); - ERROR: `some_string` is a `&` reference,
    // so the data it refers to cannot be borrowed as mutable
}

// references are immutable by default

fn change_mutable(some_string: &mut String) {
    some_string.push_str(" String value");
}

// dangling references
// -------------------

// in short: not possible in Rust, will not compile

// fn dangle() -> &String { // dangle returns a reference to a String

//     let s = String::from("hello"); // s is a new String

//     &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
//   // Danger!

// function above will not compile, error: missing lifetime specifier
// this function's return type contains a borrowed value, but there is
// no value for it to be borrowed from

// in toher words: the function returns a reference to s, which is being
// deallocated when it goes out of scope, so the reference is pointing to
// somthing that doesn't exist anymore, that's called a dangling reference

// solution is to return the String directly

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
