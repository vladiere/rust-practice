// use std::io;
//
// fn main() {
//     let mut word = String::new();
//
//     io::stdin()
//         .read_line(&mut word)
//         .expect("Failed to read line");
//
//     let result = first_letter(&word);
//     match result {
//         Some(value) => println!("The first letter of {} is {}", &word, value),
//         None => println!("No word provided"),
//     }
// }
//
// fn first_letter<'a>(word: &'a str) -> Option<char> {
//     word.chars().nth(0)
// }

// use practice::Config;
// use std::{env, process};
//
// fn main() {
//     let config = Config::build(env::args()).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {err}");
//         process::exit(1);
//     });
//
//     println!("Searching for {}", config.query);
//     println!("In file {}", config.file_path);
//
//     if let Err(e) = practice::run(config) {
//         println!("Application error: {e}");
//         process::exit(1);
//     }
// }

// fn parse_config(args: &[String]) -> Config {
//     let query = args[1].clone();
//     let file_path = args[2].clone();
//
//     Config { query, file_path }
// }
//
//

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }
//
// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }
//
// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }
//
//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;
//
//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }
//
// fn main() {
//     let store = Inventory {
//         shirts: vec![
//             ShirtColor::Blue,
//             ShirtColor::Red,
//             ShirtColor::Blue,
//             ShirtColor::Blue,
//             ShirtColor::Red,
//         ],
//     };
//
//     let user_pref1 = Some(ShirtColor::Blue);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref1, giveaway1
//     );
//
//     let user_pref2 = Some(ShirtColor::Red);
//     let giveaway2 = store.giveaway(user_pref2);
//     println!(
//         "The user with preference {:?} gets {:?}",
//         user_pref2, giveaway2
//     );
// }
//

// use std::thread;
//
// fn main() {
//     let list = vec![1, 2, 3];
//     println!("Before defining closure: {:?}", list);
//
//     thread::spawn(move || println!("From thread: {:?}", list))
//         .join()
//         .unwrap();
// }
//

// fn main() {
//     let x = || -> String {
//         if 5 > 9 {
//             String::from("The number is 5")
//         } else {
//             String::from("The number is 9")
//         }
//     };
//
//     let y = x();
//     println!("{y}");
// }
//

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 10,
//             height: 1,
//         },
//         Rectangle {
//             width: 3,
//             height: 5,
//         },
//         Rectangle {
//             width: 7,
//             height: 12,
//         },
//     ];
//
//     list.sort_by_key(|r| r.width);
//     println!("{:#?}", list);
// }
//

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }
//
// fn main() {
//     let mut list = [
//         Rectangle {
//             width: 30,
//             height: 10,
//         },
//         Rectangle {
//             width: 13,
//             height: 8,
//         },
//         Rectangle {
//             width: 35,
//             height: 20,
//         },
//         Rectangle {
//             width: 10,
//             height: 5,
//         },
//         Rectangle {
//             width: 25,
//             height: 15,
//         },
//     ];
//
//     let mut num_sort_operations = 0;
//
//     list.sort_by_key(|r| {
//         num_sort_operations += 1;
//         r.width
//     });
//
//     println!("{num_sort_operations}\n{:#?}", list);
// }
//
//

// iterator
// fn main() {
//     let v1 = vec![1, 4, 3, 2, 5, 6, 8];
//
//     let v1_iter = v1.iter();
//
//     for val in v1_iter {
//         println!("{val}");
//     }
// }

// fn main() {
//     let v1: Vec<i32> = vec![2, 1, 5, 6, 2, 7, 0];
//
//     let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
//     println!("{:#?}", v2);
// }

fn main() {
    println!("Hello world");
}
