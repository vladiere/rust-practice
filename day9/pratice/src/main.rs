// enum Optional<T> {
//     Some(T),
//     None,
// }
//
// #[derive(Debug)]
// struct User {
//     password: String,
//     username: String,
// }

fn main() {
    // let user1 = User {
    //     password,
    //     username: String::from("user1"),
    // };
    // let password: Optional<&String> = Optional::Some(&user1.password);
    //
    // match password {
    //     Optional::Some(pass) => println!("Your password is: {}", pass),
    //     Optional::None => println!("Password is required"),
    // }
    //
    // let User {
    //     password: pass,
    //     username: uname,
    // } = user1;
    //
    // if !pass.is_empty() {
    //     println!("Password required");
    // } else if !uname.is_empty() {
    //     println!("Username required");
    // } else {
    //     println!("Username and password required");
    // }

    let mut update_product =
        String::from("update products_table set product_price = $1, product_qty = product_qty");

    let operation = "plus";

    if operation == "plus" {
        update_product.push_str(" + $1, updated_at = CURRENT_TIMESTAMP where product_id = $1");
    } else {
        update_product.push_str(" - $1, updated_at = CURRENT_TIMESTAMP where product_id = $1");
    }

    println!("{}", update_product);
}
