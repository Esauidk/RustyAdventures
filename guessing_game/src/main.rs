use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to this wonderful guessing gaame!");
    println!("Here are the rules: You guess a number and we'll tell you if you're hot, cold, or on the mark!");
    println!("Now guess away!");

    let secret = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
        let guess : u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        println!("You guessed: {guess}");
    
        match guess.cmp(&secret) {
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("You're right! You win");
                break;
            },
            Ordering::Greater => println!("Too big!")
        }
    }
   
}
