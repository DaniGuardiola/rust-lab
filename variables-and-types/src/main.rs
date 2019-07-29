#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// constants
// ---------

const CONSTANT_1: u8 = 123; // immutable always, can be defined in outer scope
// const CONSTANT_1: u8 = 321; - ERROR: `CONSTANT_1` redefined here (no shadowing allowed)
// const mut CONSTANT_2: u8 = 123; - ERROR: cannot be mutable
// const CONSTANT_3 = 123; - ERROR: expected `:` (must have type annotations)

fn main() {
    const CONSTANT_1: u8 = 111; // can be defined in function scope
    
    // basics
    // ---------
    
    let x = 3; // immutable by default, cannot be defined in outer scope
    // x = 4; - ERROR: cannot assign twice to immutable variable
    let mut x = 5; // can be set to mutable with "mut"
    x = 6; // you can reassign a value of the same type to a mutable variable
    // x = "hi!"; - ERROR: expected integer, found reference (cannot mutate variable type)
    let x = "hey!"; // with shadowing a name can be reused
    
    // scalar types
    // ------------
    
    // btw type inference is a thing
    
    // - integer
    let int: u8 = 255; // other: i/u8/16/32/64/128 isize/usize - default: i32
    // let int: u8 = 256; - ERROR: literal out of range for `u8` (overflow)
    let int: usize = 12_345; // 64 bits if 64-bit arch, 32 bits if 32-bit arch
    let int: i32 = -1234i32; // type can be specified in number literals (except byte literal)

    // - floating-point
    let float: f32 = 2.123456; // other: f64 - default: f64

    // operations
    let sum = 5 + 2;
    // let sum = 5 + 2.1234; - ERROR: no implementation for `{integer} + {float}` (can't mix types)
    let difference = 3 - 1;
    let product = 2 * 34;
    let quotient = 45 / 5;
    let remainder = 200 % 9;
    // more operators: https://doc.rust-lang.org/book/appendix-02-operators.html#operators

    // - boolean
    let t = true;
    let f: bool = false;

    // - character
    let c1 = 'z';
    let c2 = 'ℤ';
    let cat = '😻';
    let cat = "😻"; // double quotes allowed apparently

    // compound types
    // --------------

    // - tuple
    // fixed length, different types allowed
    let tup: (i8, i16, u32) = (-2, 2672, 123341154);
    let (x, y, z) = tup; // destructuring
    let second = tup.1; // accesing by index

    // - array
    // fixed length, same type
    let a = [1, 2, 3, 4, 5];
    let a: [u8; 5] = [1, 2, 3, 4, 5]; // [type; length]
    let a = [3; 5]; // [value; times] - equivalent to [3, 3, 3, 3, 3]
    let third = a[2]; // accesing by index
    // let sixth = a[5]; - ERROR: index out of bounds (does not compile)
    let index = 5;
    // let sixth = a[index]; - ERROR: index out of bounds (compiles but then panics)
}
