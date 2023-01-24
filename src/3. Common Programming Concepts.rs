// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

// fn main() {
//     println!("Hello, rust.");
// }

// Variables

// fn main() {
//     let mut x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");

//     const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
//     println!("Three hours in seconds: {THREE_HOURS_IN_SECONDS}");
// }

// Shadowing

// fn main() {
//     let x = 5;

//     let x = x + 1;

//     {
//         let x = x * 2;
//         println!("The value of x in the inner scope is: {x}");
//     }

//     println!("The value of x in the outer scope is: {x}");
// }

// Bounds checking

// use std::io;
// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }

//

// Functions

// fn main() {
//     another_function(5);
//     print_labeled_measurement(5, 'h');
// }

// fn another_function(x: i32) {
//     println!("The value of x is: {x}");
// }

// fn print_labeled_measurement(val: i32, unit_label: char) {
//     println!("The measurement is: {val}{unit_label}");
// }

// Functions with return values

// fn main() {
//     let x = plus_one(5);

//     println!("The value of x is: {x}");
// }

// fn plus_one(x: i32) -> i32 {
//     x + 1
// }

// if Expressions

// fn main() {
//     let number = 7;

//     if number < 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }

//     // if number {
//     //     print!("this doesn't work");
//     //     // there is no boolean conversion in Rust
//     // }

//     if number != 0 {
//         println!("this will work");
//     }

//     let number = 6;

//     if number % 4 == 0 {
//         println!("number is divisible by 4");
//     } else if number % 3 == 0 {
//         println!("number is divisible by 3");
//     } else if number % 2 == 0 {
//         println!("number is divisible by 2");
//     } else {
//         println!("number is not divisible by 4, 3, or 2");
//     }
// }

// Using if in a let Statement (aka: shitty ternary)

// fn main() {
//     let condition = true;
//     let number = if condition { 5 } else { 6 };
//     // The following will not compile; the return types do not match.
//     // let number = if condition { 5 } else { "six" };

//     println!("The value of number is: {number}");
// }

// Returning Values from Loops

// fn main() {
//     let mut counter = 0;

//     let result = loop {
//         counter += 1;

//         if counter == 10 {
//             break counter * 2;
//         }
//     };

//     println!("The result is {result}");
// }

// Loop Labels

// fn main() {
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up;
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

// Conditional Loops with while

// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{number}!");

//         number -= 1;
//     }

//     println!("LIFTOFF!!!");
// }

// Looping Throough a Collection with for

fn main () {
    let a = [10, 20, 30, 40, 50];

    // old, error prone
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // better
    for element in a {
        println!("the value is: {element}");
    }
}
