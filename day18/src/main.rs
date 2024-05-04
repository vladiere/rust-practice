// fn main() {
//     let alphabets = ['a', 'E', 'Z', '0', 'x', '9', 'Y'];
//
//     // Fill the blank with `matches!` to make the code work
//     for ab in alphabets {
//         assert!(matches!(ab, 'A'..='Z' | 'a'..='z' | '0'..='9'))
//     }
//
//     println!("Success!");
// }
//
// enum MyEnum {
//     Foo,
//     Bar,
// }
//
// fn main() {
//     let mut count = 0;
//
//     let v = vec![MyEnum::Foo, MyEnum::Bar, MyEnum::Foo];
//     for e in v {
//         if matches!(e, MyEnum::Foo) {
//             // Fix the error by changing only this line
//             count += 1;
//         }
//     }
//
//     assert_eq!(count, 2);
//
//     println!("Success!");
// }

/// Generates a random passwords from a set of Alphanumeric.
// use rand::distributions::Alphanumeric;
// use rand::{thread_rng, Rng};
//
// fn main() {
//     let rand_string: String = thread_rng()
//         .sample_iter(&Alphanumeric)
//         .take(30)
//         .map(char::from)
//         .collect();
//     println!("{}", rand_string);
// }

/// Create a random passwords from a set of user-defined characters/.
fn main() {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0987654321)(*&^%$#@!";
    const PASS_LEN: usize = 30;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASS_LEN)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    println!("{}", password);
}
