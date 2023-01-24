// https://doc.rust-lang.org/book/ch05-00-structs.html

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// fn main() {
//     let mut user1 = User {
//         email: String::from("someone@example.com"),
//         username: String::from("someusername123"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("anotheremail@example.com");
// }

// example build function:
// fn build_user(email: String, username: String) -> User {
//     User {
//         email: email,
//         username: username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// Using the Field Init Shorthand

// struct User {
//     active: bool,
//     username: String,
//     email: String,
//     sign_in_count: u64,
// }

// // fn main() {
// //     let mut user1 = build_user(
// //         String::from("someone@example.com"),
// //         String::from("someusername123"),
// //     );

// //     user1.email = String::from("anotheremail@example.com");
// // }

// fn build_user(email: String, username: String) -> User {
//     User {
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// // Creating Instances From Other Instances With Struct Update Syntax

// fn main() {
//     let user1 = build_user(
//         String::from("someone@example.com"),
//         String::from("someusername123"),
//     );

//     let user2 = User {
//         email: String::from("another@example.com"),
//         ..user1
//     };
// }

// Using Tuple Structs without Named Fields to Create Different Types

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

// fn main() {
//     let _black = Color(0, 0, 0);
//     let _origin = Point(0, 0, 0);
// }

// Unit-Like Structs Without Any Fields

struct AlwaysEqual;

fn main() {
    let _subject = AlwaysEqual;
}
