#![feature(test)]
extern crate test;
extern crate regex;

extern crate betabear;

use test::Bencher;

static DICTIONARY: &'static str = include_str!("/usr/share/dict/words");

#[bench]
fn build_binary_search(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    b.iter(|| {
        let mut given_letters = given_letters.chars().collect::<Vec<char>>();
        given_letters.sort(); // For binary_search down the road

        for letter in word.chars() {
            if let Ok(index) = given_letters.binary_search(&letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_binary_search_prepare_first(b: &mut Bencher) {
    let word = "aaronaaronaaron";
    let given_letters = "noraanoraanoraa";

    b.bytes = given_letters.len() as u64;

    let mut given_letters = given_letters.chars().collect::<Vec<char>>();
    given_letters.sort(); // For binary_search down the road

    b.iter(|| {
        let mut given_letters = given_letters.clone();
        for letter in word.chars() {
            if let Ok(index) = given_letters.binary_search(&letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_binary_search_check_len_first(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    b.iter(|| {
        let mut given_letters = given_letters.chars().collect::<Vec<char>>();

        if given_letters.len() < word.chars().count() {
            return false;
        }

        given_letters.sort(); // For binary_search down the road

        for letter in word.chars() {
            if let Ok(index) = given_letters.binary_search(&letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    b.iter(|| {
        let mut given_letters = given_letters.chars().collect::<Vec<char>>();

        for letter in word.chars() {
            if let Some(index) = given_letters.iter().position(|&x| x == letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search_prepared(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    let given_letters = given_letters.chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut given_letters = given_letters.clone();

        for letter in word.chars() {
            if let Some(index) = given_letters.iter().position(|&x| x == letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search_prepared_len_check(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    let given_letters = given_letters.chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut given_letters = given_letters.clone();

        if given_letters.len() < word.chars().count() {
            return false;
        }

        for letter in word.chars() {
            if let Some(index) = given_letters.iter().position(|&x| x == letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search_prepared_and_sorted(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    let mut given_letters = given_letters.chars().collect::<Vec<char>>();
    given_letters.sort();

    b.iter(|| {
        let mut given_letters = given_letters.clone();

        for letter in word.chars() {
            if let Some(index) = given_letters.iter().position(|&x| x == letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search_with_chars_everywhere(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    let given_letters = given_letters.chars().collect::<Vec<char>>();
    let word = word.chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut given_letters = given_letters.clone();

        for letter in &word {
            if let Some(index) = given_letters.iter().position(|&x| x == *letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
fn build_trivial_search_with_sorted_chars_everywhere(b: &mut Bencher) {
    let given_letters = "noraanoraanoraa";
    let word = "aaronaaronaaron";

    b.bytes = given_letters.len() as u64;

    let mut given_letters = given_letters.chars().collect::<Vec<char>>();
    given_letters.sort();

    let mut word = word.chars().collect::<Vec<char>>();
    word.sort();

    b.iter(|| {
        let mut given_letters = given_letters.clone();

        for letter in &word {
            if let Some(index) = given_letters.iter().position(|&x| x == *letter) {
                given_letters.remove(index);
            } else {
                return false;
            }
        }

        true
    })
}

#[bench]
#[ignore]
fn search_1(b: &mut Bencher) {
    b.bytes = DICTIONARY.len() as u64;

    let letters = "noraa";
    let letters = letters.trim().chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut matches = DICTIONARY.lines()
                                    .filter(|word| betabear::build_with_letters(&letters, word))
                                    .map(|word| (word, word.chars().count()))
                                    .collect::<Vec<(&str, usize)>>();

        matches.sort_by(|a, b| b.1.cmp(&a.1));

        matches
    })
}

#[bench]
#[ignore]
fn search_1_with_regex(b: &mut Bencher) {
    b.bytes = DICTIONARY.len() as u64;

    let letters = "noraa";

    let regex = regex::Regex::new(&format!("^[{}]+$", letters))
                    .expect("String of given letters is malformed.");

    let letters = letters.trim().chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut matches = DICTIONARY.lines()
                                    .filter(|word| regex.is_match(word))
                                    .filter(|word| betabear::build_with_letters(&letters, word))
                                    .map(|word| (word, word.chars().count()))
                                    .collect::<Vec<(&str, usize)>>();

        matches.sort_by(|a, b| b.1.cmp(&a.1));

        matches
    })
}

#[bench]
#[ignore]
fn search_1_with_better_regex(b: &mut Bencher) {
    b.bytes = DICTIONARY.len() as u64;

    let letters = "noraa";

    let regex = regex::Regex::new(&format!("^[{}]{{1,{num}}}$",
                                           letters,
                                           num = letters.chars().count()))
                    .expect("String of given letters is malformed.");

    let letters = letters.trim().chars().collect::<Vec<char>>();

    b.iter(|| {
        let mut matches = DICTIONARY.lines()
                                    .filter(|word| regex.is_match(word))
                                    .filter(|word| betabear::build_with_letters(&letters, word))
                                    .map(|word| (word, word.chars().count()))
                                    .collect::<Vec<(&str, usize)>>();

        matches.sort_by(|a, b| b.1.cmp(&a.1));

        matches
    })
}
