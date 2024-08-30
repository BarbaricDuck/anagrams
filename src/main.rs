#![allow(unused_imports)]

use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{cmp::min, fs, io::{self, Write}};
use colored::*;

const CONTENTS: &str = include_str!("../src/20k.txt");

// fn read_file(file_path: &str) -> String {
//     fs::read_to_string(file_path)
//         .expect("Should have been able to read the file")
// }

fn get_random(v: Vec<&str>) -> &str {
    let mut rng = rand::thread_rng();

    *v.choose(&mut rng)
        .expect("The file is empty or contains no words")
}

fn shuffle(s: &str) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let mut rng = thread_rng();
    chars.shuffle(&mut rng);
    chars.into_iter().collect()
}

fn main() {
    // let file_path = "src/20k.txt";
    // let contents = read_file(file_path);
    let contents = CONTENTS;
    loop {
        let words: Vec<&str> = contents
            .split_whitespace()
            .collect();

        let word = get_random(words);
        
        let shuffled_word = shuffle(word);
    
        println!("Shuffled word: {}", shuffled_word);
        loop {
            print!("Enter your guess\n> ");
            io::stdout().flush().expect("Failed to flush stdout");
            
            let mut guess = String::new();
    
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");
    
            let min_length = min(shuffled_word.chars().count(), guess.chars().count());
    
            for i in 0..min_length {
                let c = guess.chars().nth(i).expect("Unable to get nth char from guess");
                print!("{}", if c == word.chars().nth(i).expect("Unable to get nth char from word") {c.to_string().green()} else {c.to_string().red()});
            }
            print!("\n");

            if guess.trim() == word {
                break;
            }
        }

        println!("You got it!");
        println!("Again? [y/n]");
        let mut proceed = String::new();
        io::stdin()
                .read_line(&mut proceed)
                .expect("Failed to read line");
            
        if proceed.to_lowercase().trim() != "y" {
            break
        }
    }
}