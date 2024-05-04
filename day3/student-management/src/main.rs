use std::collections::HashMap;

#[derive(Debug)]
struct People {
    name: String,
    age: u8,
    gender: String,
}

fn main() {
    let mut map = HashMap::new();

    let people1 = People {
        name: String::from("people 1"),
        age: u8::from(20),
        gender: String::from("male"),
    };
    let people2 = People {
        name: String::from("people 2"),
        age: u8::from(23),
        gender: String::from("female"),
    };
    map.insert(1, people1);
    map.insert(2, people2);

    for (key, value) in map.iter() {
        println!(
            "ID: {}\t\tName: {}\t\tAge: {}\t\tGender: {}",
            key, value.name, value.age, value.gender
        );
    }
}
