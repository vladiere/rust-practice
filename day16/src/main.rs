use std::io;

fn main() {
    let mut array1 = [1, 2, 3, 4, 5, 6, 7, 8, 9];

    println!("Original array");
    println!("{array1:?}");

    reversed_array(&mut array1);

    println!("Reversed array");
    println!("{array1:?}");

    println!("\nPyramid");
    pyramid_string(5);

    println!("\nLeft pyramid");
    pyramid_left(5);
    println!("\nRight pyramid");
    pyramid_right(5);

    println!("\nDiamond");
    diamond(3);

    println!("\nSeparate with comma");
    split_wit_comma();
    sum_of_digits();
}

fn reversed_array(arr: &mut [i32]) {
    let len = arr.len();

    for i in 0..len / 2 {
        let j = len - 1 - i;

        let temp = arr[i];
        arr[i] = arr[j];
        arr[j] = temp;
    }
}

fn pyramid_string(rows: usize) {
    for i in 0..rows {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }
}

fn pyramid_left(rows: usize) {
    for i in 0..rows {
        for _ in 0..(i + 1) {
            print!("*");
        }
        println!();
    }
}

fn pyramid_right(rows: usize) {
    for i in 0..rows {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }

        for _ in 0..(i + 1) {
            print!("*");
        }
        println!();
    }
}

fn diamond(rows: usize) {
    for i in 0..rows {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }
        println!();
    }

    for i in (0..rows - 1).rev() {
        for _ in 0..(rows - i - 1) {
            print!(" ");
        }

        for _ in 0..(2 * i + 1) {
            print!("*");
        }

        println!();
    }
}

fn split_wit_comma() {
    let sample_arr = "9,8,7,6,5,4,3,2,1";

    let resut_array: Vec<i32> = sample_arr.split(',').map(|s| s.parse().unwrap()).collect();

    println!("{resut_array:?}");
}

fn sum_of_digits() {
    println!("Sum of digits");

    loop {
        println!("Enter number (comma separated)");

        let mut input_string = String::new();

        io::stdin()
            .read_line(&mut input_string)
            .expect("Failed to read line");

        let result_numbers: Result<Vec<i32>, _> = input_string
            .trim()
            .split(',')
            .map(|s| s.trim().parse())
            .collect();

        match result_numbers {
            Ok(numbers) => {
                println!("{numbers:?}");
                let mut total = 0;
                for element in numbers {
                    total += element;
                }
                println!("Total: {total}");
                break;
            }
            Err(_) => {
                println!("Invalid input. Enter numbers separated by commas.");
                continue;
            }
        }
    }
}
