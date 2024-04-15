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
        println!("inner z = {}", z) // 99
    }
    println!("outer z = {}", z); // 5

    // * Use lib
    greet("Jacky");
    let r = rand::thread_rng().gen_range(0, 100);
    println!("r = {}", r);

    // * Tuple
    let info = (1, 3.3, 999);
    println!("info is ({}, {}, {})", info.0, info.1, info.2);
    let (jets, fuel, ammo) = info;
    println!("jets = {}, fuel = {}, ammo = {}", jets, fuel, ammo);

    // * Array
    let buf: [u8; 3] = [1,2,3];
    println!("array: {}, {}, {}", buf[0], buf[1], buf[2]);

    // * Control Flow
    for num in [7, 8, 9, 10].iter() {
        if *num > 8 {
            println!("num = {}. Break.", num);
            break
        }
    }

    let array = [(1,2), (3,4)];
    for (x, y) in array.iter() {
        print!("x = {}, y = {}\n", x, y)
    }

    // * References & Borrowing
    let s1 = String::from("abc");
    print_stuff(&s1);

    fn print_stuff(s: &String) {
        println!("print_stuff: s is {}", s);
    }

    let mut s2 = String::from("abc");
    do_stuff(&mut s2); // pass reference of s2 value to do_stuff

    let s3 = String::from("qwer");
    fn do_stuff2(s: String) {
        println!("{}", s);
    }
    do_stuff2(s3);
    // ! println!("{}", s3); // Error: s3 borrowed after move

    // Reference is immutable by default
    // x: &mut i32, *x is a mutable i32
    // x: &i32, *x is an immutable i32
    // Exactly one mutable reference OR any number of immutable references
    fn do_stuff(s: &mut String) {
        // do_stuff borrows the reference of s value

        // No need to dereference like (*s), because . operator auto dereference
        s.insert_str(0, "Hi, ");
        println!("do_stuff: s is {}", s);
    }

    // * Enum
    enum Color {
        Red,
        Green,
        Blue,
    }

    let color = Color::Red;
    //println!("color is {}", color);

    enum DispenserItem {
        Empty,
        Ammo(u8),
        Things(String, i32),
        Place{x: i32, y: i32},
    }
    use DispenserItem::*;
    let item1 = Empty;
    let item2 = Ammo(6);
    let item3 = Things("asdf".to_string(), 123); 
    let item4 = Place { x: 123, y: 456 };

    //if let Some(x) = Color::Red {
    //    println!("color is red.");
    //}

    // * Option
    let mut x = None;
    x = Some(5);
    println!("x is some: {}", x.is_some());

    // * Result
    use std::fs::File;
    let res = File::open("foo");
    //if res.is_ok() { // make sure unwrap() never crash
    //    let f = res.unwrap();
    //OR: let f = res.expect("error message");
    //}
    match res {
        Ok(f) => {
            f = res.unwrap();
        },
        Err(e) => {
            println!("Open file error: {e}");
        },
    }

}
