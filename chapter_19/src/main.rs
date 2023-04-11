fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let (a, b) = v.split_at_mut(3);

    println!("a : {:?}, b: {:?}", a, b);
    println!("v : {:?}", v);

    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    let add = returns_closure();
    println!("add: {}",add(3));
}

fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
