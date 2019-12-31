use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;
use std::process::Command;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let output = Command::new("clear").output().unwrap_or_else(|e| {
        panic!("failed to execute process: {}", e)
    });

    loop {
        println!("{}", String::from_utf8_lossy(&output.stdout));

        println!("Guess the number!");

        print!("Please input your guess: ");

        io::stdout().flush().unwrap();

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
