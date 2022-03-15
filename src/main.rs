use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret = rand::thread_rng().gen_range(1..101);

    loop {
        
        println!(" (secret# is: [{}])", secret);

        println!("s guess!!!! blablalbla");
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("failed to read line");
        
        let guess: u32 = match guess
        .trim()
        .parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input!...");
                continue;
            },
        }; 
        

        //
        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);

        println!("you guessed [{}]", guess);

        println!("you guessed [{}]", guess);
        println!("you guessed [{}]", guess);

        
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("Yay! you win!");
                break;
            },
        }
    }
}
