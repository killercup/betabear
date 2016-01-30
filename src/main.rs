extern crate betabear;

use std::env;

fn main() {
    let mut args = env::args().skip(1);

    let letters = args.next().expect("Please tell me what letters you have");

    let dict_file_path = args.next().unwrap_or("/usr/share/dict/words".into());
    let dict_file = read_file(&dict_file_path);

    let matches = betabear::search_for_words(&letters, dict_file.lines());

    for (word, letters_used) in matches {
        println!("{} ({})", word, letters_used);
    }
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
