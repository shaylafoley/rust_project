use std::io;
use std::io::Write;

fn main() {
    let words = ["apple", "banana", "orange", "grape", "melon"];

    println!("Welcome to Hangman!");

    // Pick a word manually by index 
    let secret = words[3]; // <-- change this number to pick a different word
    let secret_chars: Vec<char> = secret.chars().collect();

    let mut guessed: Vec<char> = Vec::new();
    let mut wrong_guesses = 0;
    let max_wrong = 6;

    loop {
        println!("\nWord: {}", masked_word(&secret_chars, &guessed));
        println!("Wrong guesses: {}/{}", wrong_guesses, max_wrong);

        // Check lose
        if wrong_guesses >= max_wrong {
            println!("You lost! The word was: {}", secret);
            break;
        }

        // Check win
        if secret_chars.iter().all(|c| guessed.contains(c)) {
            println!("You won! The word was: {}", secret);
            break;
        }

        // Get input
        print!("Enter a letter: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input = input.trim().to_lowercase();

        if input.len() != 1 {
            println!("Please enter exactly one letter.");
            continue;
        }

        let ch = input.chars().next().unwrap();

        if guessed.contains(&ch) {
            println!("You already guessed '{}'", ch);
            continue;
        }

        guessed.push(ch);

        if !secret_chars.contains(&ch) {
            wrong_guesses += 1;
            println!("'{}' is not in the word.", ch);
        } else {
            println!("Good guess!");
        }
    }
}

fn masked_word(secret: &[char], guessed: &[char]) -> String {
    secret
        .iter()
        .map(|c| if guessed.contains(c) { *c } else { '_' })
        .collect()
}
