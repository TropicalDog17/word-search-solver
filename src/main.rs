use std::path::Path;
use word_search_solver::board::{Board, Direction};
use word_search_solver::trie::Trie;
use word_search_solver::utils::{fetch_board, fetch_target_words};
fn main() {
    // Reading input from files
    let board_file_path = Path::new("src/input/board.txt");
    let target_words_file_path = Path::new("src/input/words.txt");
    let target_words = fetch_target_words(target_words_file_path);

    // Vec<String> to Vec<&str>
    let target_words = target_words.iter().map(String::as_str).collect();
    let letters: Vec<Vec<char>> = fetch_board(board_file_path);

    // Initialize structs
    let mut trie = Trie::new();
    trie.insert_words(&target_words);
    let board: Board = Board::new(&letters);

    // Iterate through every letters in the board and try to extend in a particular direction
    // Move on next iteration if find a word this direction or the prefix no longer belong to any word in the list
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
}
