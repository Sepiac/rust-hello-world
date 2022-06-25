use rand::Rng;
use std::io;
use std::ops;

fn main() {
    let secret_number = generate_secret_number(0..1000);

    println!("What do you think the secret number is?");
    let mut guessed_number = get_guessed_number();

    while secret_number != guessed_number {
        if guessed_number > secret_number {
            println!("Incorrect! Try lower.")
        } else if guessed_number < secret_number {
            println!("Incorrect! Try higher.")
        }
        guessed_number = get_guessed_number();
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
