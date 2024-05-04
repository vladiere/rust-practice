use std::io;

#[derive(Debug, Clone)]
enum Privilege {
    SUPER,
    STAFF,
    TECH,
    NA,
}

fn main() {
    let mut username = String::new();
    let mut password = String::new();
    let mut privil = String::new();

    println!("Enter username:");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line");

    println!("Enter password:");
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    println!("Enter who are you?");
    io::stdin()
        .read_line(&mut privil)
        .expect("Failed to read line");
    let privilege = match privil.trim().to_uppercase().as_str() {
        "SUPER" => Privilege::SUPER,
        "STAFF" => Privilege::STAFF,
        "TECH" => Privilege::TECH,
        _ => Privilege::NA,
    };

    match privilege {
        Privilege::SUPER => {
            println!("{}", username);
            if username.trim().to_string() == "super1".to_string()
                && password.trim().to_string() == "super321".to_string()
            {
                println!("You have logged in as SUPER");
            } else {
                println!("username and password incorrect");
            }
        }
        Privilege::STAFF => {
            if username.trim().to_string() == "staff1".to_string()
                && password.trim().to_string() == "staff321".to_string()
            {
                println!("You have logged in as STAFF");
            } else {
                println!("username and password incorrect");
            }
        }
        Privilege::TECH => {
            if username.trim().to_string() == "tech1".to_string()
                && password.trim().to_string() == "tech321".to_string()
            {
                println!("You have logged in as TECH");
            } else {
                println!("username and password incorrect");
            }
        }
        Privilege::NA => {
            println!("Not found who you are.");
        }
    }
}
