fn c<F>(f: F)
where
    F: FnOnce() + 'static,
{
    f();
}

fn main() {
    let mut v = vec![1, 2, 3, 4];

    c(move || {
        v.push(5);
    });

    // v.push(6);
}
