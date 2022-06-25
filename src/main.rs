use rand::Rng;
use std::io;
use std::ops;

struct NumberGame {
    secret_number: i32,
    guessed_number: i32,
}

impl NumberGame {
    fn init(&mut self) {
        println!("What do you think the secret number is?");
        self.guessed_number = get_guessed_number()
    }
    fn play(&mut self) {
        self.init();
        while self.secret_number != self.guessed_number {
            if self.guessed_number > self.secret_number {
                println!("Incorrect! Try lower.")
            } else if self.guessed_number < self.secret_number {
                println!("Incorrect! Try higher.")
            }
            self.guessed_number = get_guessed_number();
        }
    
        println!("You have correctly guessed the number!");
    }
}

fn main() {
   let mut ng = NumberGame{
    secret_number: generate_secret_number(1..101),
    // TODO(sepiac): figure out how to leave this uninitialized
    guessed_number: -1,
   };
   ng.play();
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
