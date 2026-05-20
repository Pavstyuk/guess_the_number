use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("🎲 Welcome to the game \x1b[1m'Guess a number between 1 and 100'\x1b[0m");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("Enter your answer:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // Convert the string to a number, handling the error
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Error! Please enter a number!");
                continue; // Proceed to the next loop iteration
            }
        };
        // Check that the number is in the range 1..100
        if guess < 1 || guess > 100 {
            println!("Error! The number must be between 1 and 100!");
            continue;
        }
        if guess == 0 {
            // 0 is out of range
            println!(
                "😢 You gave up! The number you guessed was {}",
                secret_number
            );
            break;
        }
        attempts += 1;
        // Compare with the hidden number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!(
                "📈 Higher! The hidden number is greater than \x1b[31m{}\x1b[0m",
                guess
            ),
            Ordering::Greater => println!(
                "📉 Lower! The hidden number is less than \x1b[31m{}\x1b[0m",
                guess
            ),
            Ordering::Equal => {
                println!(
                    "\n🎉 \x1b[1mCongratulations!\x1b[0m You guessed the number \x1b[32m{}\x1b[0m. Number of attempts: \x1b[34m{}\x1b[0m",
                    secret_number, attempts
                );
                println!("🏆 Thanks for playing!");
                break; // Exit the loop
            }
        }
    }
}
