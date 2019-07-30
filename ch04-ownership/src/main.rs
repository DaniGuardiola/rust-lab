#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
fn main() {
    // variable scope
    // --------------

    {
        // s is not valid here, it’s not yet declared
        let s = "hello"; // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    // the String type
    // ---------------

    let mut s = "hello"; // string literal (&str) is immutable, stored in the binary

    // s.push_str(", world!"); - ERROR: no method named `push_str` found
    // for type `&str` in the current scope (not mutable)

    let mut s = String::from("hello"); // String type is mutable, stored in the heap
    s.push_str(", world!"); // possible to append to String

    // move
    // ----

    // values stored in the heap must allocate memory on the heap
    // Rust automatically frees the memory when variable goes out of scope

    // String values are allocated in the heap, with a pointer that is allocated
    // in the stack containing the memory address, length and capacity

    let s1 = String::from("hello");

    // when assigned to other variable, the memory remains the same and
    // only the pointer is copied

    let s2 = s1;

    // however the first variable (s1) in not valid anymore to prevent
    // a double free error

    // println!("{}, world!", s1); - ERROR: value used here after move

    // so it's not a deep copy, nor a shallow copy, it's a *move*
    // s1 has moved into s2

    // Rust will never automatically create “deep” copies of data

    // clone
    // -----

    let s1 = String::from("hello");
    let s2 = s1.clone(); // deep copy

    // copy
    // ----

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    // code is valid because it is not moving, even though it's not
    // cloning either, because types such as integers that have a known
    // size at compile time are stored entirely on the stack, so copies
    // of the actual values are quick to make

    // copy trait is present in:
    // - As a general rule, any group of simple scalar values
    // - All the integer types, such as u32
    // - The Boolean type, bool, with values true and false
    // - All the floating point types, such as f64
    // - The character type, char
    // - Tuples, if they only contain types that are also Copy. For
    //   example, (i32, i32) is Copy, but (i32, String) is not

    // ownership and functions
    // -----------------------

    // passing a variable to a function will move or copy
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    // println!("{}, world", s); - ERROR: value borrowed here after move

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward

    println!("x: {}", x);

    // return values and scope
    // -----------------------

    let s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into
                                       // takes_and_gives_back, which also
                                       // moves its return value into s3
} // - ownership and functions: here, s3 goes out of scope and is dropped. s2
  // goes out of scope but was moved, so nothing happens. s1 goes out of scope
  // and is dropped.
  // - return values and scope: here, x goes out of scope, then s. But because
  // s's value was moved, nothing special happens.

// ownership and functions
// -----------------------

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

// return values and scope
// -----------------------

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}
