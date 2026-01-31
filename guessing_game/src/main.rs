use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // rand::thread_rng function that gives us the particular random number generator we’re going to use: one that is local to the current thread of execution and is seeded by the operating system. 
    // gen_range method that generates a random number in the given range

    let mut guess = String::new();

    println!("Please input your guess.");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    // trim method is used to remove the whitespace from the string
    // parse method is used to parse the string into a number
    // expect method is used to handle the error if the string is not a number

    match guess.cmp(&secret_number){
        // here cmp (comparing) is a method that we compare the refrence of anaything here "guess" with the secret number (& symbol denotes the refrence of secret number)
        // it returns a variant of ordering enum and we use match expression to decide what to do in each case possible.
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),

    }

    io::stdin()
        .read_line(&mut guess) 
        // the full job of read_line method is to read a line from the standard input and append it to the given string
        // expect is a method that is called on the result of the read_line method
        // if the result is Ok, it will return the value inside the Ok
        // if the result is Err, it will return the error message  
        //  The & indicates that the argument is a reference to the variable guess 
        .expect("Failed to read line");



    println!("You guessed: {guess}");
}