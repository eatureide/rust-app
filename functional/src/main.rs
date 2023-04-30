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


    let mut s1 = String::from("hello");
    {
        let s2 = &mut s1;
    }

    let s3 = &mut s1;

}
