// fn main() {
//     let num = {
//         let a: u8 = 32;
//         // 返回值
//         a
//     };

//     test(num, '🤔');

//     let test_func: u8 = test(num, '😎');

//     println!("{}", test_func)
// }

// fn test(x: u8, string: char) -> u8 {
//     println!("x = {} -- str = {}", x, string);

//     // 返回值
//     1 + 2
// }

fn main() {
    let s1 = test();
}

fn test() -> String {
    let res = String::from("hello");
    res
}
