fn main() {
    for element in (0..5).rev() {
        println!("{element}");
    }
}

// use std::io;
//
// fn main() {
//     let mut index = String::new();
//
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Index is a number");
//
//     let index: usize = index.trim().parse().expect("Not a number");
//     println!("The month is {}", get_month(index));
// }
//
// fn get_month(index: usize) -> String {
//     let months = [
//         "January",
//         "February",
//         "March",
//         "April",
//         "May",
//         "June",
//         "July",
//         "August",
//         "September",
//         "October",
//         "November",
//         "December",
//     ];
//
//     let month = if index != 0 {
//         months[index - 1]
//     } else {
//         months[index]
//     };
//     return month.to_string();
// }
