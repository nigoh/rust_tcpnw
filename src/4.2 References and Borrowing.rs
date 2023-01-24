// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

// The String Type

// #![allow(unused)]
// fn main() {
//     // A string literal (slice).
//     let s = "hello1";

//     // A String on the heap.
//     let s2 = String::from("hello2");

//     // A String can be mutated.
//     let mut s3 = String::from("Hello");
//     s3.push_str(", world!");

//     println!("{}", s);
//     println!("{}", s2);
//     println!("{}", s3);
// }

// Ownership and Functions

// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html

// References and Borrowing

// fn main() {
//   let s1 = String::from("hello");

//   let len = calculate_length(&s1);

//   println!("The length of '{}' is {}.", s1, len);
// }

// fn calculate_length(s: &String) -> usize {
//   s.len()
// }

// This code does not compile:
// fn main() {
//   let s = String::from("hello");

//   change(&s);
// }

// fn change(some_string: &String) {
//   some_string.push_str(", world");
// }

// // have to borrow as a mutable reference:
// fn main() {
//   let mut s = String::from("hello");

//   change(&mut s);
// }

// fn change(some_string: &mut String) {
//   some_string.push_str(", world");
// }

// Dangling References

// will not compile due to a reference to nothing being returned:
// fn main() {
//     let reference_to_nothing = dangle();
// }

// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }

// the solution is to just return the string directly:
// (Ownership is moved out)
fn main() {
    let _string = no_dangle();
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
