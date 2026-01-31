use std::io;

fn main(){
    println!("Guess the number!");
    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess) 
        // the full job of read_line method is to read a line from the standard input and append it to the given string
        // expect is a method that is called on the result of the read_line method
        // if the result is Ok, it will return the value inside the Ok
        // if the result is Err, it will return the error message  
        //  The & indicates that the argument is a reference to the variable guess.which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times.
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}