// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }
//
// fn main() {
//     let rect1 = Rectangle {
//         width: 40,
//         height: 20,
//     };
//
//     let rect2 = Rectangle {
//         width: 30,
//         height: 50,
//     };
//
//     println!(
//         "The area of the rectangle 1 is {} and the width is {}",
//         rect1.area(),
//         rect1.width
//     );
//
//     println!(
//         "The area of the rectangle 2 is {} and the width is {}",
//         rect2.area(),
//         rect2.width
//     );
// }

// enums
//

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let mut home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1"),
    // };

    home.kind = IpAddrKind::V6;
    home.address = String::from("::1");

    println!("Home address: {:#?}", home);
    // println!("Loop back address: {:?}", loopback);
}
