
//stadard input
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    //prompt user
    println!("Guess the number");

    //generate random number (inclusive lower, exclusive upper bound)
    //thread_rng uses current thread seeded by os
    let secret_number = rand::thread_rng().gen_range(1,101);
   
    //allow multiple guesses
    loop{
    
        println!("Please input your guess");
        let mut guess = String::new();

        //read stdin to var
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        /*
            convert guess to an u32
            Rust allows to "shadow" a previous value of guess
            This allows us to not have to create two variables for same intent
            Trim eliminates any white space, parse to specified type uautomagically
            Handle parse return (which is a Result enum of Ok or Err) with match
        */
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num, //ok returns num
            Err(_) => continue, //err returns many things. the underscore is catch-all value
        };

        println!("You guessed {}",guess);

        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            //break loop
            Ordering::Equal => {
                println!("Correct! You win. Thank you for playing");
                break;
            }
        }

    }//end loop

    
    
}
