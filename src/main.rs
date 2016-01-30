extern crate regex;

use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let letters = args.next().expect("Please tell me what letters you have");

    let dict_file_path = args.next().unwrap_or("/usr/share/dict/words".into());
    let dict_file = read_file(&dict_file_path);

    let matches = search_for_words(&letters, &dict_file);

    for (word, letters_used) in matches {
        println!("{} ({})", word, letters_used);
    }
}

/// Filter dictionary by words that can be written with the given letters
///
/// Returns a vector of valid words.
fn search_for_words<'l, 'dict>(letters: &'l str,
                               dictionary: &'dict str)
                               -> Vec<(&'dict str, usize)> {
    let regex = regex::Regex::new(&format!("^[{}]+$", letters))
                    .expect("String of given letters is malformed.");

    let mut matches = dictionary.lines()
                                .filter(|word| regex.is_match(word))
                                .filter(|word| build_with_letters(&letters, word))
                                .map(|word| (word, word.chars().count()))
                                .collect::<Vec<(&str, usize)>>();

    // Sort the vec so that the words with the most letters used are on top
    matches.sort_by(|a, b| b.1.cmp(&a.1));

    matches
}

/// Checks whether word can be written with the given letters.
fn build_with_letters(given_letters: &str, word: &str) -> bool {
    let mut given_letters = given_letters.chars().collect::<Vec<char>>();
    given_letters.sort(); // For binary_search down the road

    for letter in word.chars() {
        if let Ok(index) = given_letters.binary_search(&letter) {
            // We used a letter, remove it from the stash.
            given_letters.remove(index);
        } else {
            // Word uses a letter we don't have (any more)
            return false;
        }
    }

    true
}

/// Read a file to a string, panic on error
fn read_file(file_name: &str) -> String {
    use std::io::Read;
    use std::fs::File;

    let mut file = File::open(file_name).expect(&format!("Error opening file {}", file_name));
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect(&format!("Error reading file {}", file_name));

    content
}
