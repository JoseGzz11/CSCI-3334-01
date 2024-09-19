// Implements a function check_guess(guess: i32, secret: i32) -> i32
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess < secret {
        1
    }
    else if guess > secret {
        -1
    }
    else {
        0
    }
}

fn main() {
    // Use a mutable variable to store a "secret" number
    let secret_num: i32 = 6;
    let _num_guesses: i32 = 0;
    let mut guess = 0;

    loop {
        let check = check_guess(guess, secret_num);

        if check == 1 {
            println!("Guess was too high.");
        }
        else if check == -1 {
            println!("Guess was too low.");
        }
        else {
            println!("Guess was correct!.");
            break;
        }
        guess += 1;
    }
}
