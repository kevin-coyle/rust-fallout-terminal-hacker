use std::io::{self, Write};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn main() {
    let mut words = vec![];
    println!("Enter possible words (type 'done' to finish):");
    loop {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        let word = word.trim().to_uppercase();
        if word == "DONE" {
            break;
        }
        if !word.is_empty() {
            words.push(word);
        }
    }

    if words.is_empty() {
        println!("No words entered. Exiting.");
        return;
    }

    println!("Possible words are: {:?}", words);

    loop {
        let mut rng = thread_rng();
        let guess = words.choose(&mut rng).expect("Word list is empty").clone();
        println!("Guessing: {}", guess);

        print!("Enter likeness for '{}': ", guess);
        io::stdout().flush().unwrap();

        let mut likeness_input = String::new();
        io::stdin().read_line(&mut likeness_input).expect("Failed to read line");
        let likeness: usize = match likeness_input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid likeness value. Exiting.");
                return;
            }
        };

        words = filter_words(&words, &guess, likeness);
        if words.len() == 1 {
            println!("The password is: {}", words[0]);
            break;
        } else if words.is_empty() {
            println!("No possible words left. Exiting.");
            break;
        } else {
            println!("Remaining possible words: {:?}", words);
        }
    }
}

fn filter_words(words: &[String], guess: &str, likeness: usize) -> Vec<String> {
    words
        .iter()
        .filter(|&word| calculate_likeness(word, guess) == likeness)
        .cloned()
        .collect()
}

fn calculate_likeness(word1: &str, word2: &str) -> usize {
    word1
        .chars()
        .zip(word2.chars())
        .filter(|(c1, c2)| c1 == c2)
        .count()
}
