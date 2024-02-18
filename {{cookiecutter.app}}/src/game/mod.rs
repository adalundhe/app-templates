
pub mod assets;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use assets::GuessResult;


pub struct Game {
    pub secret_number: u32,
    pub current_guess: u32,
    pub max_range: u32,
    pub tries: i32,
    pub guesses: Vec<u32>
    
}

impl Default for Game {

    fn default() -> Self {

        let max_range = 10;

        Game {
            secret_number: rand::thread_rng()
                .gen_range(1..=max_range),
            current_guess: 0,
            max_range,
            tries: 3,
            guesses: Vec::new()
        }
    }
}

impl Game {

    fn guess_number(&mut self){
        println!("Please input your guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut  guess)
            .expect("Failed to read line");
    
        self.current_guess = guess.trim()
            .parse()
            .unwrap_or_else(|_| {
                println!("That's not a number!");
                0
            });
    
    }

    fn check_guess(&mut self) -> GuessResult {
        
        if self.current_guess < 1 || self.current_guess > self.max_range {
            return GuessResult::INVALID;
        }

        if self.guesses.contains(&self.current_guess) {
            return GuessResult::REPEAT;
        }

        match self.current_guess.cmp(&self.secret_number) {
            Ordering::Less => {
                self.tries -= 1;
                self.guesses.push(self.current_guess);
                GuessResult::LESS

            },
            Ordering::Greater => {
                self.tries -= 1;
                self.guesses.push(self.current_guess);
                GuessResult::GREATER

            },
            Ordering::Equal => {
                GuessResult::WIN
            }
        }
    }

    pub fn run_game(&mut self){

        let max_range = self.max_range;
        println!("Guess a number between 1 and {max_range}!");

        while self.tries > 0 {
    
            self.guess_number();
            let result = self.check_guess();
            let guess = self.current_guess;

            match result {
                GuessResult::GREATER => println!("Too large!"),
                GuessResult::LESS => println!("Too small!"),
                GuessResult::REPEAT => println!("You already guessed - {guess}!"),
                GuessResult::INVALID => println!("Oops! That either isn't a number or is out side of the max range of 1 to {max_range}"),
                GuessResult::WIN => {
                    println!("You win!");
                    break;
                }
            }
            
        }

        if self.tries <= 0 {
            println!("Oh no! You're out of tries!")
        }
    }
    
}