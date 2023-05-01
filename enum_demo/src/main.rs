// #[derive(Debug)]

// enum UsState {
//     Alabama,
//     Alaska,
// }

// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter(UsState),
// }

// fn value_in_cents(coin: Coin) -> u8 {
//     match coin {
//         Coin::Penny => {
//             println!("penny!");
//             1
//         }
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter(state) => {
//             println!("state from {:?}!", state);
//             25
//         }
//     }
// }

// fn plus_one(x: Option<i32>) -> Option<i32> {
//     match x {
//         None => None,
//         Some(i) => Some(i + 1),
//     }
// }

fn main() {
    let v = Some(0u8);

    println!("{:?}", v);
    
    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }

    // let v = 0u8;
    // match v {
    //     1 => println!("one"),
    //     2 => println!("two"),
    //     _ => (),
    // }

    // println!("{}", v);

    // let five = plus_one(Some(50));
    // print!("{:?}", five);
    // enum IpAddrKind {
    //     V4,
    //     V6,
    // }

    // struct Ipaddr {
    //     kind: IpAddrKind,
    //     address: String,
    // }

    // let four = IpAddrKind::V4;
    // let six = IpAddrKind::V6;
    // route(four);
    // route(six);
    // route(IpAddrKind::V6);

    // fn route(ip_kind: IpAddrKind) -> IpAddrKind {
    //     ip_kind
    // }

    // enum Message {
    //     Quit,
    //     Move { x: i32, y: i32 },
    //     Write(String),
    //     ChangeColor(i32, i32, i32),
    // }

    // impl Message {
    //     fn call(&self) {
    //         // print!("{:?}", &self);
    //     }
    // }

    // let q = Message::Quit;
    // let m = Message::Move { x: 12, y: 22 };
    // let w = Message::Write(String::from("hello"));
    // let c = Message::ChangeColor(0, 20, 30);

    // c.call()

    // let some_number = Some(5);
    // let some_string = Some("A String");

    // let absend_number: Option<i32> = None;

    // let c = Coin::Quarter(UsState::Alabama);
    // println!("{}", value_in_cents(c));
}
