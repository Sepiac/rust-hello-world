use rand::Rng;
use std::{io, ops};

pub struct NumberGame {
    guessed_number: i32,
    secret_number_range: ops::Range<i32>,
}

impl NumberGame {
    pub fn new(secret_number_range: ops::Range<i32>) -> Self {
        Self {
            secret_number_range,
            ..Default::default()
        }
    }

    fn init(&mut self) -> i32 {
        println!("What do you think the secret number is?");
        self.guessed_number = Self::get_guessed_number();
        self.generate_secret_number()
    }

    pub fn play(&mut self) {
        let secret_number = self.init();

        while secret_number != self.guessed_number {
            if self.guessed_number > secret_number {
                println!("Incorrect! Try lower.")
            } else if self.guessed_number < secret_number {
                println!("Incorrect! Try higher.")
            }
            self.guessed_number = Self::get_guessed_number();
        }

        println!("You have correctly guessed the number!");
    }

    fn generate_secret_number(&self) -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(self.secret_number_range.clone())
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
        Self {
            guessed_number: -1,
            secret_number_range: 1..10
        }
    }
}
