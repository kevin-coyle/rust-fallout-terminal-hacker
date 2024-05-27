use std::io::{self, Write};
use std::time::Duration;
use std::thread;
use rand::seq::SliceRandom;
use rand::thread_rng;
use console::Term;
use rodio::{OutputStream, Sink, Decoder};
use std::io::Cursor;

use colored::*;


fn main() {
    let mut words = vec![];
    let term = Term::stdout();
    term.clear_screen().unwrap();
    let robco_notice = r#"ROBCO INDUSTRIES (TM) TERMLINK PROTOCOL"#;

    slow_print(&robco_notice.on_green().to_string(), 25);
    if !std::env::args().any(|arg| arg == "--no-sound") {
        play_sound("ok").unwrap();
    }
    let instructions = r#"Enter possible words (type 'done' to finish):"#;
    slow_print(&instructions.green().to_string(), 25);
    loop {
        let mut word = String::new();
        io::stdin().read_line(&mut word).expect("Failed to read line");
        let word = word.trim().to_uppercase();
        if word == "DONE" {
            break;
        }
        if !word.is_empty() {
            words.push(word);
            if !std::env::args().any(|arg| arg == "--no-sound") {
                play_sound("bad").unwrap();
            }
        }
    }

    if words.is_empty() {
        let no_words = r#"No words entered. Exiting."#;
        println!("{}", no_words.green());
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
            if !std::env::args().any(|arg| arg == "--no-sound") {
                play_sound("ok").unwrap();
            }
            break;
        } else if words.is_empty() {
            println!("No possible words left. Exiting.");
            break;
        } else {
            println!("Remaining possible words: {:?}", words);
        }
    }
}

fn play_sound(file_path: &str) -> Result<rodio::Sink, Box<dyn std::error::Error>> {
    let ok = include_bytes!("ui_hacking_passgood.wav");
    let fail = include_bytes!("ui_hacking_passbad.wav");
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;
    let src_file: &[u8] = match file_path {
        "ok" => ok,
        _ => fail,
    };
    let source = Decoder::new_wav(Cursor::new(src_file))?;
    sink.append(source);
    sink.play();
    thread::sleep(Duration::from_millis(200));
    Ok(sink)
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

fn slow_print(s: &str, delay: u64) {
    for c in s.chars() {
        print!("{}", c);
        io::stdout().flush().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(delay));
    }
    println!();
}
