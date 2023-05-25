use rand::{thread_rng, Rng};
use std::cmp::Ordering;

pub fn guessing_game() {
    let mut random_number: i32 = thread_rng().gen_range(1..=100);

    for _ in 0..100 {
        println!("{}", random_number);
        for j in 0.. {
            let mut guess = 100 / 2;

            match guess.cmp(&random_number) {
                Ordering::Greater => guess /= 2,
                Ordering::Less => guess *= 2,
                Ordering::Equal => println!("Correct guess ({}) matched in {} iterations", guess, j + 1)
            }
        }
        random_number = thread_rng().gen_range(1..=100);
    }
}
