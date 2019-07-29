#![allow(unused_variables)]
#![allow(unreachable_code)]

fn main() {
    // calling functions
    // -----------------

    another_function(12_345_678, -78_543_21);
    // can be defined in any position (below or above)

    // statements vs. expressions
    // --------------------------

    // statements: do not return a value
    let x = 5; // variable creation/assignment: statements

    // let y = (let x = 5); - ERROR: expected expression, found statement

    // function definitions are also statements

    // expressions: return a value
    let x = 5; // literals (5) are expressions
    let x = 5 + 6; // operations are expressions

    let y = {
        // scope-creating blocks ({}) can be expressions
        let x = 3;
        x + 1 // no semicolon makes it an expression
              // with a semicolon, the block would become a statement
    };

    println!("Result of the five function: {}", five());
    println!("Result of the early_return function: {}", early_return());
}

fn another_function(x: i32, y: i32) {
    // mandatory parameter type annotations
    println!("another_function is being called! x: {}, y: {}", x, y);
}

fn five() -> i8 {
    // return type, if any, must be specified
    5 // like in any scope-creating block ({}), when semicolon is omitted in last line, value is returned
}

fn early_return() -> i8 {
    return 8; // use the "return" keyword to return early
    5 // this line doesn't actually make sense, but you get it
}
