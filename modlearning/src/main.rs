
use calculator::calculator;

struct Test {
    pub name: String,
    pub mask: i32,
}

impl Test {
    pub fn PrintAddr(&self) {
        println!("{}, {}", self.name, self.mask)
    }
}

fn main() {
    println!("Hello, world!");
    let x: i32 = 1;
    let y: i32 = 2;
    let z = calculator::add(x, y);
    println!("{:?}", z);

    let t = Test {
        name: String::from("john"),
        mask: 255,
    };

    t.PrintAddr()

}
