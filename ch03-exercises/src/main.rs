// exercises suggested here: https://doc.rust-lang.org/book/ch03-05-control-flow.html#summary

// fahrenheit <--> celsius conversion
// ----------------------------------

const FACTOR: f64 = 1.8;
const DIFFERENCE: f64 = 32.0;

fn fahrenheit_to_celsius(t: f64) -> f64 {
    (t - DIFFERENCE) / FACTOR
}

fn print_fahrenheit_to_celsius(degrees: f64) {
    println!("{}°F => {}°C", degrees, fahrenheit_to_celsius(degrees));
}

fn celsius_to_fahrenheit(t: f64) -> f64 {
    t * FACTOR + DIFFERENCE
}

fn print_celsius_to_fahrenheit(degrees: f64) {
    println!("{}°C => {}°F", degrees, celsius_to_fahrenheit(degrees));
}

// fibonacci
// ---------

fn fibonacci(n: u32) -> u128 {
    let mut counter = 0;
    let mut n1 = 0;
    let mut n2 = 1;
    let mut acc = n1;

    while counter < n {
        if counter == 0 {
            acc = 1;
        } else {
            acc = n1 + n2;
            n1 = n2;
            n2 = acc;
        }
        counter += 1;
    }

    acc
}

fn print_fibonacci(n: u32) {
    println!("{}: {}", n, fibonacci(n));
}

// The Twelve Days Of Christmas
// ----------------------------

// const NUMBERS: (str, str, str, str, str, str, str, str, str, str, str) = ("first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eigth", "ninth", "tenth", "eleventh", "twelfth");

// fn the_twelve_days_of_christmas() {}

// TODO
// honestly f**k the twelve days of christmas shove them up your a**

// run and log this shit
// ---------------------

fn main() {
    // fahrenheit <--> celsius
    println!("// fahrenheit <--> celsius\n// -----------------------\n");
    let temperatures: [f64; 12] = [
        -120.0, -20.0, -5.0, 0.0, 10.0, 25.0, 38.0, 48.0, 60.0, 73.0, 90.0, 120.0,
    ];
    for t in temperatures.iter() {
        print_fahrenheit_to_celsius(*t);
        print_celsius_to_fahrenheit(*t);
        println!()
    }

    // fibonacci
    println!("\n// fibonacci\n// ---------\n");
    for n in { 1..101 } {
        print_fibonacci(n);
    }
}
