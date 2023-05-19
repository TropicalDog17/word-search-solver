use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn fetch_board(file_path: &Path) -> Vec<Vec<char>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut vec_letter = Vec::new();
            for c in line.chars() {
                if c.is_alphabetic() {
                    vec_letter.push(c);
                }
            }
            result.push(vec_letter.clone());
        }
    }

    println!("Letters: {:?}", result);
    result
}

pub fn fetch_target_words(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut result = Vec::new();
    for word in contents.split(" ") {
        result.push(word.to_owned());
    }
    result
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::utils::fetch_board;
    #[test]
    fn ensure_board_input_exists() {
        let file_path = Path::new("src/input/board.txt");
        fetch_board(file_path);
    }
    #[test]
    fn ensure_target_words_input_exists() {
        let file_path = Path::new("src/input/words.txt");
        fetch_board(file_path);
    }
}
