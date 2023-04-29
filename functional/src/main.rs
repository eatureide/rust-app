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

    let arr = 1..4;
    for element in arr {
        println!("{}", element)
    }
}
