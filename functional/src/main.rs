// fn main() {
//     println!("Hello, world!");
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//     println!("{},{}", x, y);
// }

// fn plus_five(x: i32) -> i32 {
//     // return 5;
//     return x + 5;
// }

fn main() {
    // let y = 5 + 6;
    // let x = 5;
    // let y = {
    //     let x = 1;
    //     x + 3
    // };
    // println!("{}", y);
    // let x = plus_five(5);
    // println!("{}", x)

    // let number = 7;
    // // if number < 5 {
    // //     println!("小于5")
    // // } else {
    // //     println!("大于5")
    // // }
    // match number % 4 {
    //     0 => println!("xxx 4"),
    //     _ => println!("no xxx4")
    // }

    // let condition = true;
    // let number = if condition { 5 } else { 6 };

    // print!("{}", number)

    // let mut count = 0;
    // loop {
    //     count += 1;

    //     if count >= 100 {
    //         break;
    //     }

    //     println!("again!{}", count);
    // }

    // let mut number = 30;
    // while number != 0 {
    //     println!("{}", number);
    //     number = number - 1;
    // }

    // println!("off");

    // let arr = 1..4;
    // for element in arr {
    //     println!("{}", element);
    // }

    // let mut s = String::from("hello");
    // s.push_str(", world");
    // println!("{}", s);

    // let s1 = String::from("hello");
    // let s2 = s1.clone();
    // println!("{}", s1);

    // let x = 5;
    // let y = x;
    // println!("{},{}", x, y);

    // let s = String::from("hello world");
    // take_ownership(s);

    // println!("{}", s);

    // let x = 5;
    // makes_copy(x);

    // fn take_ownership(some_string: String) {
    //     println!("{}", some_string);
    // }

    // fn makes_copy(some_number: i32) {
    //     println!("{}", some_number);
    // }

    // let s1 = gives_ownership();

    // let s2: String = String::from("hello");
    // let s3 = takes_and_gives_back(s2);
    // println!("{}", s3);
    // fn gives_ownership() -> String {
    //     let some_string = String::from("hello");
    //     some_string
    //     // return some_string;
    // }

    // fn takes_and_gives_back(a_string: String) -> String {
    //     println!("{}", a_string);
    //     a_string
    // }

    // let mut s1 = String::from("hello");
    // let len = calculate_length(&mut s1);

    // println!("{},{}", s1, len);

    // fn calculate_length(s: &mut String) -> usize {
    //     s.push_str(",world");
    //     s.len()
    // }

    // let mut s1 = String::from("hello");
    // {
    //     let s2 = &mut s1;
    // }

    // let s3 = &mut s1;

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // // let s1: &mut String = &mut s;

    // println!("{},{}", r1, r2);

    // let r = dangle();
    // fn dangle() -> &String {
    //     let s = String::from("hello");
    //     &s
    // }
    // let s = String::from("hello wolrd");
    // let word = first_world(&s);
    // // s.clear();
    // println!("{}", word);

    // fn first_world(s: &String) -> &str {
    //     let bytes = s.as_bytes();
    //     let key = b' ';
    //     for (i, &item) in bytes.iter().enumerate() {
    //         if item == key {
    //             print!("{}", item);
    //             return &s[..i];
    //         }
    //     }
    //     &s[..]
    // }

    // let arr = [1, 2, 3, 4];
    // for i in arr.iter() {
    //     println!("{}", i);
    // }

    // let s = String::from("hello wolrd");
    // let hello = &s[..5];
    // let world = &s[6..];
    // let whole = &s[..];

    // println!("{}", whole);
    // println!("{},{}", hello, world);

    // struct User {
    //     username: String,
    //     email: String,
    //     sign_in_count: u64,
    //     active: bool,
    // }

    // let mut user1 = User {
    //     email: String::from("abc@email.com"),
    //     username: String::from("nikky"),
    //     active: true,
    //     sign_in_count: 556,
    // };

    // fn build_user(email: String, username: String) -> User {
    //     User {
    //         email,
    //         username,
    //         active: true,
    //         sign_in_count: 0,
    //     }
    // }

    // let email = String::from("78@qq.com");
    // let username = String::from("Eature");
    // let user1 = build_user(email, username);

    // let user2 = User {
    //     email: String::from("123@.com"),
    //     username: String::from("username"),
    //     active: user1.active,
    //     ..user1
    // };
    // struct Color(i32, i32);
    // let c = Color(12, 12);

    
}
