/// Level 1
/// Challenge 3
// use std::io;
//
// fn main() {
//     let mut input_number = String::new();
//
//     println!("Enter a number");
//     io::stdin()
//         .read_line(&mut input_number)
//         .expect("Failed to read line");
//
//     let number: u32 = input_number.trim().parse().expect("Not a number");
//
//     if number % 2 == 0 {
//         println!("{} is an even number", number);
//     } else {
//         println!("{} is an odd number", number);
//     }
// }

/// Challenge 4
// fn main() {
//     let arr1 = vec![5, 4, 3, 6, 7, 8, 1, 3];
//
//     for item in arr1 {
//         println!("{item}");
//     }
// }

/// Challenge 5
// fn main() {
//     let mut height = String::new();
//     let mut width = String::new();
//
//     println!("Enter height");
//     io::stdin()
//         .read_line(&mut height)
//         .expect("Failed to read line");
//     println!("Enter width");
//     io::stdin()
//         .read_line(&mut width)
//         .expect("Failed to read line");
//
//     let height = height.trim().parse().expect("Not a number");
//     let width = width.trim().parse().expect("Not a number");
//
//     let result = calculate_area(height, width);
//     println!("{result} is the area of triangle");
// }
//
// fn calculate_area(height: f32, width: f32) -> f32 {
//     (width * height) * 0.5
// }

/// Level 2

/// Challenge 1
// #[derive(Debug, Clone, Copy)]
// struct Rectangle {
//     width: f32,
//     height: f32,
// }
//
// impl Rectangle {
//     fn area(&self) -> f32 {
//         (&self.width * &self.height) * 0.5
//     }
// }
//
// fn main() {
//     let rec1 = Rectangle {
//         height: 5.0,
//         width: 3.0,
//     };
//     println!("{} is the area", rec1.area());
// }

/// Challenge 2
/// ownership and borrowing
// fn main() {
//     let numbers = vec![5, 4, 3, 2, 7, 9, 6];
//     let result = take_ownership(numbers.to_owned());
//
//     println!("{result:?}");
//     println!("{numbers:?}");
// }
//
// fn take_ownership(mut arr: Vec<i32>) -> Vec<i32> {
//     arr.push(3);
//     arr
// }
use std::{
    fs::File,
    io::{BufReader, Read},
};

/// Challenge 3
/// reading file with custom errors.

type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Error {
    FileNotFound,
    PermissionDenied,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::result::Result<(), std::fmt::Error> {
        write!(f, "{self}")
    }
}

impl std::error::Error for Error {}

fn main() -> Result<(), Error> {
    let file = File::open("../data.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    println!("{contents:?}");
    Ok(())
}
