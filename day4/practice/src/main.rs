// use std::fmt::Display;
//
// struct Pair<T> {
//     x: T,
//     y: T,
// }
//
// impl<T> Pair<T> {
//     fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
// }
//
// impl<T: Display + PartialOrd> Pair<T> {
//     fn cmp_display(&self) {
//         if self.x >= self.y {
//             println!("The largest number is x = {}", self.x);
//         } else {
//             println!("The largest number is y = {}", self.y);
//         }
//     }
// }
//
// fn main() {
//     let test = Pair::new(10, 20);
//
//     let test2 = Pair::new(String::from("axis x"), String::from("axis y"));
//
//     println!("X is {} and Y is {}", test.x, test.y);
//     Pair::cmp_display(&test);
//     Pair::cmp_display(&test2);
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let x = "testing";
    let y = "abstract";

    let result = longest(x, y);
    println!("The largest string is {result}");
}
