use basic::greet;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    // * Variables
    let a = 123;
    let mut b: i32 = 456;
    println!("a = {}, b = {}", a, b);
    b = 789;
    println!("b = {}", b);
    const C: f64 = 1.1;
    println!("c = {}", C);

    // * Scope
    let x = 5;
    {
        let y = 99;
        println!("x = {}, y = {}", x, y)
    }
    // println!("x = {}, y = {}", x, y); // Error!

    // Shadowing
    let z = 5;
    {
        let z = 99;
        println!("inner z = {}", z)
    }
    println!("outer z = {}", z);

    // * lib
    greet("Jacky");
    let r = rand::thread_rng().gen_range(0, 100);
    println!("r = {}", r);

}
