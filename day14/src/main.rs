use std::io::{self, BufRead};

/*
 * Complete the 'fizzBuzz' function below.
 *
 * The function accepts INTEGER n as parameter.
 */

#[allow(non_snake_case)]
fn fizzBuzz(n: i32) {
    let mut i = 1;
    loop {
        if i <= n {
            if i % 3 == 0 && i % 5 == 0 {
                println!("FizzBuzz");
                i += 1;
            } else if i % 3 == 0 {
                println!("Fizz");
                i += 1;
            } else if i % 5 == 0 {
                println!("Buzz");
                i += 1;
            } else {
                println!("{i}");
                i += 1;
            }
        } else {
            break;
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    fizzBuzz(n);
}
