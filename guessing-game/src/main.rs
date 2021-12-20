use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    /* 
    ! denotes a macro. 
    variables in rust are const by default
    mut denotes a mutable variable
    */
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");

    let guess : u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    //The underscore in Err is a catch all val. Essentially, we wanna ignore any and all errors and continue the game at this point 
    /*
    the :u32 denotes what type we want the variable to be: an unsigned 32 bit integer
    trim removes the newline character from the input, parse converts a string into an int
    */

    println!("You guessed: {}", guess);

    /*
    match = expression composed of arms
    arms = pattern and code to run if parameter matches pattern
    */
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
            println!("You win!");
            break;
        }
    }
    }
}
