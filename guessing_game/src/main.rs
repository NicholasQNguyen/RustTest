use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is {}", secret_number);


    let mut guessed = false;

    while guessed == false{
        let mut guess = String::new();
        println!("Please input your guess.");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        if guess > secret_number{
            println!("Too high!");
        }
        else if guess < secret_number{
            println!("Too low!");
        }
        else if guess == secret_number{
            println!("Gratz!");
            println!("Bye!");
            guessed = true;
        }
    }

}
