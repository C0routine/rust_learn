use rand::{rng, Rng};
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("###### Guess the number! ######");
    println!("Please input your guess Number(-100 ~ 999)");

    // Random number generator
    let secret_number: i32 = rng().random_range(0..=2);
    let compare_count: u16 = compare_guess(secret_number);
    println!("You Tried {compare_count}");
}

// 비밀번호와 사용자 입력값을 비교하는 함수
fn compare_guess(secret_number: i32) -> u16 {
    let mut count: u16 = 0;
    loop {
        let mut guess: String = String::new();

        eprint!(">> ");
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input Only Number");
                continue;
            }
        };

        count += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                return count;
            }
        }
    }
}
