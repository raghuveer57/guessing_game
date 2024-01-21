use std::{io, cmp::Ordering};
use rand::Rng;
// use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is {secret_number}");

    loop{
        println!("Please input your guess:");

        let mut guess = String::new(); // mutable variable, the default behaviour is immutable

        io::stdin()     // this is something likle cin in c++
            .read_line(&mut guess)   // reads the input line into guess( should be passed as a reference)
            .expect("failed to read the line"); //failure handling

    let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) =>{
            println!("Please input a valid number");
            continue;
        },
    };

    println!("You guessed: {}",guess);
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win!!!!!");
                println!("Exiting the program");
                break;
            }
        }
    }
}
