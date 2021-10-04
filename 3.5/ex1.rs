use std::io;
fn main(){
    let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => return {
                println!("Bruh, that's text");
                ()
            }
        };
    println!("You guessed: {}", (guess as f32 * 1.8) + 32.0 );
  }
