#![allow(dead_code)]
#![allow(unused_variables)]

// enums
// -----

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddrStruct {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// enums can have methods
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn route_kind(ip_kind: IpAddrKind) {
    println!("Routing ip (kind) {:?}", ip_kind);
}

fn route_struct(ip: IpAddrStruct) {
    println!("Routing ip (struct) {:?}", ip);
}

fn route_enum(ip: IpAddr) {
    println!("Routing ip (enum) {:?}", ip);
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route_kind(four);
    route_kind(six);

    let home = IpAddrStruct {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddrStruct {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    route_struct(home);
    route_struct(loopback);

    let home = IpAddr::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));

    route_enum(home);
    route_enum(loopback);

    // Option
    // ------

    let some_number = Some(5);
    let some_string = Some("a string");

    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // let sum = x + y; - ERROR: cannot add `std::option::Option<i8>` to `i8`
}
