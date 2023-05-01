#[derive(Debug)]

enum SpreadsheetCell {
    Int(i32),
    Text(String),
    Float(f64),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3 + 1),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }
    // let mut v: Vec<i32> = Vec::new();
    // // let v = vec![1, 2, 3];
    // v.push(1);
    // v.push(2);
    // v.push(3);
    // v.push(4);

    // let mut v: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let first = &v[0];
    // v.push(6);
    // println!("{}", first);
    // let third: &i32 = &v[2];
    // for i in &mut v {
    //     *i += 50;
    // }

    // for i in &mut v {
    //     println!("{}", i);
    // }

    // match v.get(20) {
    //     Some(third) => println!("{}", third),
    //     None => println!("unget"),
    // }
}
