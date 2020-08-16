use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Hello, rusty!");

    loop {
        println!("Guess a number (type 'c' to exit): ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Faile to read line");

        let guess = guess.trim();

        if guess.eq("c") | guess.eq("C") {
            println!("Exiting...");
            break;
        }
        
        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        println!("Your guess is {}", guess);

        let secret_number = rand::thread_rng().gen_range(1, 101);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Your guess is less. Right number is {}", secret_number),
            Ordering::Greater => println!("Your guess is high. Right number is {}", secret_number),
            Ordering::Equal => {
                println!("Congratulation ! You guessed exact number");
                break;
            }
        }
    }
}
