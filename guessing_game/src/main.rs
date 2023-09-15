use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let min = 0;
    let max = 99;

    println!("Guess the number!");
    println!("Try to guess the number between {min} and {max}.");

    let secret = rand::thread_rng().gen_range(min..=max);
    let mut guess_str = String::new();

    loop {
        guess_str.clear();
        
        println!("Please input your guess...");

        io::stdin()
        .read_line(&mut guess_str)
        .expect("Failed to read line.");

        let guess: i32 = match guess_str.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let is_correct = match guess.cmp(&secret) {
            Ordering::Less => { 
                println!("Too small!");
                false
            },
            Ordering::Greater => { 
                println!("Too big!");
                false
            },
            Ordering::Equal => { 
                println!("Correct!");
                true
            }
        };

        if is_correct {
            break;
        }
    }
}
