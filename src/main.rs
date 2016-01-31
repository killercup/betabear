extern crate betabear;
extern crate lines;

use std::env;
use std::io::Read;
use std::fs::File;
use lines::linereader::LineReader;

fn main() {
    let mut args = env::args().skip(1);

    let letters = args.next().expect("Please tell me what letters you have");

    let dict_file_path = args.next().unwrap_or("/usr/share/dict/words".into());
    let dict_file = StrLines::new(&dict_file_path).unwrap();

    let matches = betabear::search_for_words(&letters, dict_file.iter());

    for (word, letters_used) in matches {
        println!("{} ({})", word, letters_used);
    }
}

struct StrLines {
    reader: LineReader<File>
}

impl<'file> StrLines {
    pub fn new(file_name: &str) -> Result<StrLines, std::io::Error> {
        let mut file = try!(File::open(file_name));

        Ok(StrLines { reader: LineReader::new(file) })
    }

    pub fn iter(&'file mut self) -> StrLinesIterator<'file> {
        StrLinesIterator { lines: self }
    }
}

struct StrLinesIterator<'file> {
    lines: &'file mut StrLines,
}

impl<'file> Iterator for StrLinesIterator<'file> {
    type Item = &'file str;

    fn next(&'file mut self) -> Option<Self::Item> {
        match self.lines.reader.read_line() {
            Ok(b) if b.is_empty() => {
                return None;
            }
            Ok(line) => {
                use std::str::from_utf8_unchecked;
                return Some(unsafe { from_utf8_unchecked(line) });
            }
            Err(e) => {
                // No good way to handle this error, let's just explode
                panic!("Error reading file: {}", e);
            }
        }
    }
}
