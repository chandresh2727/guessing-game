//Guessing game 
use rand ::{thread_rng,Rng};
use std::io;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to guessing game universe ");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Secrete number {}",secret_number);
    println!("Please enter your choice: ");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess);
    //    .expect("Failed to read line");
    println!("Your guessed:{} ",guess);
    let guess:u32 = guess.trim().parse().expect("Type an integer");
    // println!("{}",guess + 1);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too Big"),
        Ordering::Equal => {
            println!("You win"); 
            // break;
        },
    }

}
