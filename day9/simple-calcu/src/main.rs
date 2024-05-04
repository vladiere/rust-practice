use std::io;

fn main() {
    println!("Simple calculator");

    let operation = choose_operation();
    let (x, y) = x_and_y();

    result_number(operation, x, y);
}

fn result_number(operation: u32, x: u32, y: u32) {
    if operation == 1 {
        println!("answer: {}", addition(x, y));
    } else if operation == 2 {
        println!("answer: {}", division(x, y));
    } else if operation == 3 {
        println!("answer: {}", multiplication(x, y));
    } else if operation == 4 {
        println!("answer: {}", subtraction(x, y));
    } else {
        println!("Wrong choice");
    }
}

fn choose_operation() -> u32 {
    println!("\nChoose\n[1] Addition\n[2] Division\n[3] Multiplication\n[4] Subtraction");
    let mut operation = String::new();

    io::stdin()
        .read_line(&mut operation)
        .expect("Failed to read line");

    println!("Choice:");
    let operation: u32 = match operation.trim().parse() {
        Ok(x) => x,
        Err(_) => 0,
    };

    operation
}

fn x_and_y() -> (u32, u32) {
    let mut n1 = String::new();
    let mut n2 = String::new();
    println!("First number:");
    io::stdin().read_line(&mut n1).expect("Failed to read line");
    println!("Second number:");
    io::stdin().read_line(&mut n2).expect("Failed to read line");

    let num1: u32 = match n1.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let num2: u32 = match n2.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    (num1, num2)
}

fn addition(x: u32, y: u32) -> i32 {
    x as i32 + y as i32
}

fn division(x: u32, y: u32) -> f32 {
    if x != 0 || y != 0 {
        x as f32 / y as f32
    } else {
        0.0
    }
}

fn multiplication(x: u32, y: u32) -> u32 {
    x * y
}

fn subtraction(x: u32, y: u32) -> i32 {
    x as i32 - y as i32
}
