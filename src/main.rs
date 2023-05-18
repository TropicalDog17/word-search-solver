use std::fmt::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::vec;
use word_search_solver::board::{Board, Direction};
use word_search_solver::trie::{Trie, TrieNode};
fn main() -> Result<(), io::Error> {
    let mut trie: Trie = Trie::new();
    trie.insert_words(&vec!["CPLUSPLUS", "HASKELL", "JAVASCRIPT"]);
    let file_path = Path::new("src/words.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut letters: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut vec_letter = Vec::new();
            for c in line.chars() {
                if c.is_alphabetic() {
                    vec_letter.push(c);
                }
            }
            letters.push(vec_letter.clone());
        }
    }

    println!("Letters: {:?}", letters);
    let board: Board = Board::new(&letters);
    println!(
        "{:?}",
        board.get_string_from_direction(10, 2, &Direction::Right, 7)
    );
    let mut result: Vec<String> = Vec::new();
    for i in 0..board.get_rows() {
        for j in 0..board.get_cols() {
            for d in Direction::iterator() {
                let mut dist: i32 = 0;
                let mut prefix = board.get_string_from_direction(i, j, d, dist).unwrap();
                while trie.starts_with(&prefix) {
                    if trie.search(&prefix) {
                        result.push(prefix.clone());
                        break;
                    } else {
                        dist += 1;
                        if let Some(p) = board.get_string_from_direction(i, j, d, dist) {
                            prefix = p;
                            continue;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("{:?}", result);

    Ok(())
}
