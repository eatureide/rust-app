use std::collections::HashMap;

fn main() {
    // let teams = vec![String::from("blue"), String::from("yellow")];
    // let initial_scores = vec![10, 50];

    // let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    // println!("{:?}", scores);

    let name = String::from("fav color");
    let value = String::from("blue");
    let mut map = HashMap::new();
    map.insert(&name, &value);
    println!("{},{}", name, value);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("blue"), 10);
    // scores.insert(String::from("yellow"), 50);

    // let s1 = String::from("hello, ");
    // let s2 = String::from("world");

    // // let s3 = s1 + &s2;
    // let s3 = s1 + "-" + &s2;
    // // 这里是交出了所有权，所以后面s1就不能再使用了

    // println!("{}", s1);
    // println!("{}", s2);
    // println!("{}", s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // // let s3 = s1 + "-" + &s2 + "-" + &s3;

    // let res = format!("{}{}{}", s1, s2, s3);
    // println!("{}", res);

    // let s = String::from("tic_tac");
    // // println!("{:?}", s.len());
    // for b in s.chars() {
    //     println!("{}", b);
    // }

    // let hello = "tic_tac";
    // let s = &hello[0..4];
    // println!("{}", s);
}
