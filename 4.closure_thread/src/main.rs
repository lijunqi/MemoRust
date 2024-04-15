fn main() {
    println!("====== Closure & Thread ======");

    // ================= Closure ====================
    // 1
    let add = |x, y| {x+y};
    println!("add(1,2) = {}", add(1,2));

    // 2
    let mut v = vec![2, 4, 6];

    v.iter()
     .map(|&x| x * 3)
     .filter(|&x| x > 10)
     .fold(0, |acc, x| acc + x);

    for i in v {
        println!("i = {i}");
    }

    // ================= Thread ====================
    use::std::thread;
    let handle = thread::spawn(move || {
        // do stuff in a child thread
        let x = 1;
    });

    // do stuff simulataneousy in the main thread

    // wait untill thread has exited
    handle.join().unwrap();

}
