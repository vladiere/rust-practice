// fn main() {
//     let some_str = "Ramen";
//
//     let reversed = some_str.chars().rev().collect::<String>();
//     println!("{reversed}");
// }
//
// fn main() {
//     let x = leap_year_match_on_tuple(1997);
//     println!("{x}");
// }
//
// fn leap_year_match_on_tuple(year: i64) -> bool {
//     match (year % 4, year % 100, year % 400) {
//         (_, _, 0) => true,
//         (_, 0, _) => false,
//         (0, _, _) => true,
//         _ => false,
//     }
// }
//
// fn main() {
//     let input = "9142";
//     let length = 2;
//     let output = another_series(input, length);
//     let expected = &["91", "14", "42"];
//     assert_eq!(output, expected);
// }
//
// fn series(digits: &str, len: usize) -> Vec<String> {
//     let mut strings = Vec::new();
//
//     if digits.len() < len {
//         return strings;
//     }
//
//     for i in 0..=digits.len() - len {
//         strings.push(digits[i..i + len].to_string());
//     }
//
//     strings
// }
//
// fn another_series(digits: &str, len: usize) -> Vec<String> {
//     if len == 0 {
//         return vec!["".to_string(); digits.len() + 1];
//     }
//
//     digits
//         .chars()
//         .collect::<Vec<char>>()
//         .windows(len)
//         .map(|w| w.iter().collect())
//         .collect()
// }

fn main() {
    println!("{}", nth(4));
}

fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }

    let mut count = 0;
    let mut num = 3;

    while count < n {
        let mut is_prime = true;

        for i in 2..=(num as f64).sqrt() as u32 {
            if num % i == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            count += 1;
            if count == n {
                return num;
            }
        }

        num += 2;
    }

    unreachable!();
}
