use std::io;

fn main() {
    let mut guess: String = String::new();

    println!("Guess the number!");
    println!("Please input your guess.");

    io::stdin()
        .read_line(&mut guess) // adding the buffer_data to the guess variable
        .expect("Failed to read line");
}
