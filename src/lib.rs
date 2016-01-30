extern crate regex;

/// Checks whether word can be written with the given letters.
///
/// ```rust
/// let letters = ['a', 'p', 'p', 'l', 'e', 'x', 'y', 'z'];
/// let word = "apple";
///
/// assert!(betabear::build_with_letters(&letters, word));
/// ```
pub fn build_with_letters(given_letters: &[char], word: &str) -> bool {
    let mut given_letters = given_letters.to_vec();

    for letter in word.chars() {
        // We could use binary search here, but it doesn't matter performance wise for the small
        // words (i.e. element counts) we are dealing with. Also, we would need to ensure that the
        // given letters were sorted.
        if let Some(index) = given_letters.iter().position(|&x| x == letter) {
            // We used a letter, remove it from the stash.
            given_letters.remove(index);
        } else {
            // Word uses a letter we don't have (any more)
            return false;
        }
    }

    true
}

/// Filter dictionary by words that can be written with the given letters
///
/// Returns a vector of valid words.
///
/// ```rust
/// let letters = "applepie";
/// let dictionary = vec!["apple", "pie", "beer"];
///
/// let solution = betabear::search_for_words(&letters, dictionary.into_iter());
///
/// assert_eq!(solution, vec![("apple", 5), ("pie", 3)]);
/// ```
pub fn search_for_words<'l, 'dict, D>(letters: &'l str, dictionary: D) -> Vec<(&'dict str, usize)>
    where D: Iterator<Item = &'dict str>
{
    let regex = regex::Regex::new(&format!("^[{}]+$", letters))
                    .expect("String of given letters is malformed.");

    let letters = letters.trim().chars().collect::<Vec<char>>();

    let mut matches = dictionary.filter(|word| regex.is_match(word))
                                .filter(|word| build_with_letters(&letters, word))
                                .map(|word| (word, word.chars().count()))
                                .collect::<Vec<(&str, usize)>>();

    // Sort the vec so that the words with the most letters used are on top
    matches.sort_by(|a, b| b.1.cmp(&a.1));

    matches
}

#[test]
fn find_trivial_word() {
    let letters = "banana";
    let words = vec!["banana"];

    assert_eq!(search_for_words(letters, words.into_iter()),
               vec![("banana", 6)]);
}

#[test]
fn find_in_order() {
    let letters = "banana";
    let words = vec!["banana", "ban"];

    assert_eq!(search_for_words(letters, words.into_iter()),
               vec![("banana", 6), ("ban", 3)]);
}
