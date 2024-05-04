mod models;

use models::post::PostBody;
use models::user::UserBody;

fn main() {
    let user1 = UserBody {
        username: String::from("test123"),
        password: String::from("password123"),
        fullname: String::from("testing user"),
        age: 23,
    };

    let post1 = PostBody {
        title: "The one that got away",
        content: "Lorem ipsum dolor sit amet, qui minim labore adipisicing minim sint cillum sint consectetur cupidatat.",
        user_id: 1,
        posted_at: "Jan 1 2024"
    };

    println!("{:#?}", user1);
    println!("{:#?}", post1);
}
