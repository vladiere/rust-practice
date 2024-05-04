// #[derive(Debug)]
// enum Option<T> {
//     None,
//     Some(T),
// }
//
// fn main() {
//     let some_number = Option::Some(20);
//     let absent_number: Option<i32> = Option::None;
//     println!("Some number {:#?}, {:#?}", some_number, absent_number);
// }

// #[derive(Debug)]
// enum Coin {
//     Penny,
//     Nickel,
//     Dime,
//     Quarter,
// }
//
// fn value_in_cents(coin: Coin) -> i8 {
//     match coin {
//         Coin::Penny => 1,
//         Coin::Nickel => 5,
//         Coin::Dime => 10,
//         Coin::Quarter => 25,
//     }
// }
//
// fn main() {
//     let coin = value_in_cents(Coin::Quarter);
//     println!("Coin is {}", coin);
// }

// use std::io;
//
// #[derive(Debug)]
// enum Sex {
//     Male,
//     Female,
//     Other,
// }
//
// #[derive(Debug)]
// struct People {
//     gender: String,
//     name: String,
//     age: u32,
// }
//
// fn define_people() -> (String, u32, String) {
//     let mut gender = String::new();
//     let mut name = String::new();
//     let mut age = String::new();
//
//     println!("Enter your name:");
//     io::stdin()
//         .read_line(&mut name)
//         .expect("Failed to read line");
//
//     println!("Enter your age:");
//     io::stdin()
//         .read_line(&mut age)
//         .expect("Failed to read line");
//
//     println!("Enter gender:");
//     io::stdin()
//         .read_line(&mut gender)
//         .expect("Failed to read line");
//
//     let age: u32 = age.trim().parse().expect("Not a number");
//
//     (name, age, gender)
// }
//
// fn check_gender(sex: String) -> String {
//     if sex == "male" {
//         String::from("You are an alpha")
//     } else if sex == "female" {
//         String::from("You are a queen")
//     } else {
//         String::from("naahhh")
//     }
// }
//
// fn check_age(age: u32) -> String {
//     let mut result = String::from("You are an ");
//     if age > 18 {
//         result.push_str("adult");
//     } else {
//         result.push_str("still young");
//     }
//
//     result
// }
//
// fn main() {
//     let (name, age, gender) = define_people();
//     let people1 = People { gender, name, age };
//
//     let result_gender = check_gender(people1.gender);
//     let result_age = check_age(people1.age);
//
//     println!("Hi! {}, {} and {}", people1.name, result_age, result_gender);
// }

// use std::fs::File;
// use std::io::ErrorKind;
//
// fn main() {
//     let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
//         if error.kind() == ErrorKind::NotFound {
//             File::create("hello.txt").unwrap_or_else(|error| {
//                 panic!("Problem creating the file: {:?}", error);
//             })
//         } else {
//             panic!("Problem opening the file: {:?}", error);
//         }
//     });
//
//     println!("{:?}", greeting_file);
// }

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![23, 54, 13, 451, 560];

    let result = largest(&number_list);
    println!("The largest number is {}", result);
}
