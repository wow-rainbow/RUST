use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    // --snip--

    println!("guess the number!");
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("input your number.");

        let mut guess= String::new();

        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
    
    
    

}
