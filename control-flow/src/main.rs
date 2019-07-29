#![allow(unused_variables)]
fn main() {
    let number = 3;

    // if / else
    // ---------

    if number < 5 {
        println!("lower than five");
    } else if number == 5 {
        println!("equal to five");
    } else {
        println!("greater than five");
    }

    // condition must always evaluate to boolean
    // no falsy / truthy values

    let result = if number < 5 {
        "lower"
    } else {
        // 5 - ERROR: expected reference, found integer
        // (all values returning from all branches must
        // be the same type)
        "not lower"
    };

    // lack of semicolons make the block being executed an
    // expression returning that value which is then assigned
    // to the variable

    // loops
    // -----

    // loop {
    //     println!("infinite loop")
    // }
    // loop with no exit condition will be stuck forever

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // loop will stop and the value of the
                               // expression to the right will be
                               // returned from it
        }
    };

    // while
    // -----

    let mut number = 3;

    while number != 0 {
        // loop will keep going until
        // condition is not true anymore
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // for
    // ---

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element)
    }

    // using a Range
    for number in (1..4).rev() {
        // rev() reverses the range
        println!("{}!", number);
    }

    println!("LIFTOFF!!!");
}
