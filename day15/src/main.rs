use argonautica::Hasher;

fn main() {
    let pass = "hello world";

    let mut hasher = Hasher::default();
    let hash = hasher
        .with_password(pass)
        .with_secret_key("OWL-project-2024")
        .hash()
        .unwrap();

    println!("{}", hash);
}
