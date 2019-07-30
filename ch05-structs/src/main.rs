#![allow(dead_code)]
#![allow(unused_variables)]

// struct definition
struct User {
    // fields with their data types
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    // struct instance creation
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // instance can be mutable
    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    println!("User email: {}", user2.email); // values can be accesed with dot notation

    user2.email = String::from("anotheremail@example.com"); // values can be changed in mutable instances

    // can use values from other instances
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    // with "..instance" shorthand (fills all remaining values)
    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    // tuple structs:

    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // tuple struct instances behave like tuples
}

// functions can return structs
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// shorthands make it more readable
fn build_user_shorthands(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
