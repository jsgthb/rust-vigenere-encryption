use std::{process, collections::HashMap};

use dialoguer::Input;

fn main() {
    // Define alphabet and lookup table
    let alphabet: Vec<char> = ('a'..='z').into_iter().collect::<Vec<char>>();
    let mut alphabet_lookup: HashMap<&char, i32> = HashMap::new();
    let mut count = 0;
    for letter in alphabet.iter() {
        alphabet_lookup.insert(letter, count);
        count += 1;
    }
    
    // Get vigenere key
    let mut key : String = Input::new()
    .with_prompt("Enter vigenere key")
    .with_initial_text("schaack")
    .interact_text().unwrap();
    key = key.to_lowercase();
    let key_chars: Vec<char> = key.chars().collect();
    // Exit program if input has numeric values
    for letter in key.chars() {
        if letter.is_numeric() {
            println!("Please enter letters only");
            // Exit prompt
            let exit : String = Input::new()
            .with_prompt("Press enter to exit")
            .allow_empty(true)
            .interact_text().unwrap();
            process::exit(0x0100);
        }
    }

    // Get plaintext
    let mut plaintext : String = Input::new()
    .with_prompt("Enter plaintext")
    .with_initial_text("she sells sea shells by the sea shore")
    .interact_text().unwrap();
    plaintext = plaintext.to_lowercase();
    for letter in plaintext.chars() {
        if letter.is_numeric() {
            println!("Please enter letters only");
            let exit : String = Input::new()
            .with_prompt("Press enter to exit")
            .allow_empty(true)
            .interact_text().unwrap();
            process::exit(0x0100);
        }
    }

    // Encrypt plaintext
    let mut key_position: usize = 0;
    let key_length = key_chars.len();
    print!("\nCiphertext: ");
    for letter in plaintext.chars() {
        if letter.is_whitespace() {
            print!("{}", letter)
        } else {
            let plaintext_index: usize = alphabet_lookup[&letter].try_into().unwrap();
            let key_index: usize = alphabet_lookup[&key_chars[key_position]].try_into().unwrap();
            print!("{}", alphabet.get((plaintext_index + key_index) % 26).unwrap().to_string());
            key_position += 1;
            key_position = key_position % key_length;
        }
    }
    print!("\n");

    let exit : String = Input::new()
            .with_prompt("Press enter to exit")
            .allow_empty(true)
            .interact_text().unwrap();
}
