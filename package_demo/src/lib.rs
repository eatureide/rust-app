// // // mod front_of_house {
// // //     pub mod hosting {
// // //         pub fn add_to_waitlist() {
// // //             println!("hi")
// // //         }
// // //     }

// // //     // mod serving {
// // //     //     fn take_order() {}
// // //     //     fn serve_order() {}
// // //     //     fn take_payment() {}
// // //     // }
// // // }

// // // pub fn eat_at_restaurant() {
// // //     front_of_house::hosting::add_to_waitlist();
// // // }

// // // fn serve_order() {}

// // // mod back_of_house {
// // //     fn fix_incorrect_order() {
// // //         cook_order();
// // //         super::serve_order();
// // //         crate::serve_order();
// // //     }

// // //     fn cook_order() {}
// // // }

// // mod back_of_house {
// //     pub struct Breakfast {
// //         pub toast: String,
// //         pub seasonal_fruit: String,
// //     }

// //     pub enum Appetizer {
// //         Soup,
// //         Salad
// //     }

// //     impl Breakfast {
// //         pub fn summber(toast: &str) -> Breakfast {
// //             Breakfast {
// //                 toast: String::from(toast),
// //                 seasonal_fruit: String::from("peaches"),
// //             }
// //         }
// //     }
// // }

// // pub fn eat_at_restaurant() {
// //     let mut meal = back_of_house::Breakfast::summber("rye");
// //     meal.toast = String::from("weaht");
// //     println!("i d like {} toast olease", meal.toast);
// //     meal.seasonal_fruit = String::from("blueberries");
// // }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//         pub fn some_function() {}
//     }
// }

// use crate::front_of_house::hosting;
// // use front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     front_of_house::hosting::some_function();
//     hosting::add_to_waitlist();
//     hosting::some_function();
// }

// // use std::fmt::Result;
// // use std::io::Result as IoResult;

// // fn f1() -> IoResult {}
// // fn f1() -> Result {}

// // fn main() {}

use std::collections::*;
use std::io::{self, Write};

fn main() {}
