
struct RedFox {
    enemy: bool,
    life: u32,
}

impl RedFox {
    fn new(e: bool, l: u32) -> Self {
        Self {
            enemy: e,
            life: l,
        }
    }
}

// For implementing generic function
trait Noisy {
    fn get_noise(&self) -> &str;
}

impl Noisy for RedFox {
    fn get_noise(&self) -> &str {
        "Meow?"
    }
}

fn print_noise<T: Noisy>(item: T) {
    println!("Noise: {}", item.get_noise());
}

impl Noisy for u8 {
    fn get_noise(&self) -> &str {
        "BYTE!"
    }
}

// 2.
trait Run {
    fn run(&self) {
        println!("I'm running.");
    }
}

struct Robot {}
impl Run for Robot {}

fn main() {
    let fox1 = RedFox {enemy: true, life: 70};
    println!("Hello fox1, enemy[{}] life[{}]!", fox1.enemy, fox1.life);
    let fox2 = RedFox::new(false, 123);
    println!("Hello fox2, enemy[{}] life[{}]!", fox2.enemy, fox2.life);

    println!("fox1 noise: {}", fox1.get_noise());
    print_noise(fox2);
    print_noise(5_u8);

    let bot = Robot{};
    bot.run();
}
