struct Cat;
struct Dog;
struct Duck;
struct Bird;

enum Pet {
    Cat(Cat),
    Dog(Dog),
    Duck(Duck),
    Bird(Bird),
}

trait Noise {
    fn noise(&self);
}

impl Noise for Cat {
    fn noise(&self) {
        println!("Meoww..")
    }
}
impl Noise for Dog {
    fn noise(&self) {
        println!("Arrff...")
    }
}
impl Noise for Duck {
    fn noise(&self) {
        println!("Quaackk..")
    }
}
impl Noise for Bird {
    fn noise(&self) {
        println!("Tweeett..")
    }
}

fn main() {
    println!("Hello, world!");
}
