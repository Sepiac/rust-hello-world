use rand::Rng;
use core::num;
use std::io;
use std::ops;

struct NumberGame {
    secret_number: i32,
    guessed_number: i32,
}

impl NumberGame {
    fn init(&mut self) {
        println!("What do you think the secret number is?");
        self.guessed_number = Self::get_guessed_number()
    }
    fn play(&mut self) {
        self.init();
        while self.secret_number != self.guessed_number {
            if self.guessed_number > self.secret_number {
                println!("Incorrect! Try lower.")
            } else if self.guessed_number < self.secret_number {
                println!("Incorrect! Try higher.")
            }
            self.guessed_number = Self::get_guessed_number();
        }

        println!("You have correctly guessed the number!");
    }
    fn generate_secret_number(range: ops::Range<i32>) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(range)
    }

    fn get_guessed_number() -> i32 {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read number from user");
        buffer.trim().parse::<i32>().unwrap()
    }
}

impl Default for NumberGame {
    fn default() -> Self {
        Self { secret_number: Self::generate_secret_number(1..10), guessed_number: -1 }
    }
}

fn main() {
    let mut ng = NumberGame {
        ..Default::default()
    };
    ng.play();
}
