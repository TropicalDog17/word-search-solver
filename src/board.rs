use crate::trie::Trie;
use std::slice::Iter;
pub struct Board {
    pub letters: Vec<Vec<char>>,
    cols: usize,
    rows: usize,
}
impl Board {
    pub fn new(letters: &Vec<Vec<char>>) -> Self {
        let cols = letters.get(0).unwrap().len();
        let rows = letters.len();
        Board {
            letters: letters.to_owned(),
            cols,
            rows,
        }
    }
    /// Get all possible words in that position, then append all the words found to the result vector
    ///
    /// # Arguments
    ///
    /// * `i` - The row index of the position
    /// * `j` - The column index of the position
    /// * `result` - The vector to store the result
    /// * `board` - The board to search
    /// * `trie` - The trie to search
    ///
    /// # Examples
    /// ```
    ///
    /// use word_search_solver::board::Board;
    /// use word_search_solver::trie::Trie;
    /// use std::collections::HashSet;
    /// let board = Board::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    /// let mut trie = Trie::new();
    /// trie.insert_words(&vec!["abc", "adg", "ghi"]);
    /// let mut result: Vec<String> = Vec::new();
    /// let mut result_idx: Vec<(usize, usize)> = Vec::new();
    /// board.get_all_possible_word(0, 0, &mut result, &mut result_idx, &board, &trie);
    /// let expected_result: HashSet<&str> = HashSet::from(["abc", "adg"]);
    /// let expected_result_idx: HashSet<(usize, usize)> = HashSet::from([(0, 2), (0, 6)]);
    /// assert!(expected_result.contains(result[0].as_str()));
    /// assert!(expected_result.contains(result[1].as_str()));
    /// println!("{:?}", result_idx);
    /// assert!(expected_result_idx.contains(&result_idx[0]));
    /// assert!(expected_result_idx.contains(&result_idx[1]));
    /// ```
    ///
    ///

    pub fn get_all_possible_word(
        &self,
        i: usize,
        j: usize,
        result: &mut Vec<String>,
        result_idx: &mut Vec<(usize, usize)>,
        board: &Board,
        trie: &Trie,
    ) {
        // Check if a given position is valid
        for d in Direction::iterator() {
            let mut dist: i32 = 1;
            let mut prefix = board
                .get_string_from_direction(i, j, d, dist)
                .unwrap_or("".to_string());

            if prefix.is_empty() {
                continue;
            }
            while trie.starts_with(&prefix) {
                if trie.search(&prefix) {
                    // Get the start and end index of the prefix in the grid
                    let start = (i, j);
                    let end = (
                        Board::add(i, d.to_coord_diff().0 * dist).unwrap_or_default(),
                        Board::add(j, d.to_coord_diff().1 * dist).unwrap_or_default(),
                    );
                    result.push(prefix.clone());
                    println!("{:?}", result);
                    result_idx.push((
                        start.0 * board.get_cols() + start.1,
                        end.0 * board.get_cols() + end.1,
                    ));
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
    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_cols(&self) -> usize {
        self.cols
    }

    /// Get the letter in the board at a given position, retrun None if the position is invalid or out of bound
    ///
    /// # Arguments
    ///
    /// * `x` - The row index of the position
    /// * `y` - The column index of the position
    ///
    ///
    /// # Examples
    /// ```
    /// use word_search_solver::board::Board;
    /// let board = Board::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    /// assert_eq!(board.get_letter(Some(0), Some(0)), Some("a".to_string()));
    /// assert_eq!(board.get_letter(Some(0), Some(1)), Some("b".to_string()));
    /// assert_eq!(board.get_letter(Some(0), Some(2)), Some("c".to_string()));
    /// assert_eq!(board.get_letter(None, Some(0)), None);
    /// assert_eq!(board.get_letter(Some(0), None), None);
    /// assert_eq!(board.get_letter(None, None), None);
    /// assert_eq!(board.get_letter(Some(3), Some(0)), None);
    /// ```
    ///

    pub fn get_letter(&self, x: Option<usize>, y: Option<usize>) -> Option<String> {
        if x == None || y == None {
            return None;
        }
        match (x, y) {
            (Some(x), Some(y)) => {
                if let Some(row) = self.letters.get(x) {
                    if let Some(letter) = row.get(y) {
                        return Some(letter.to_string());
                    }
                }
                None
            }
            _ => return None,
        }
    }
    /// Get the string in the board from a given position and direction
    /// # Arguments
    /// * `start_x` - The row index of the position
    /// * `start_y` - The column index of the position
    /// * `direction` - The direction to search
    /// * `distance` - The distance to search, if = 1 then return the letter at the position
    /// # Examples
    /// ```
    /// use word_search_solver::board::Board;
    /// use word_search_solver::board::Direction;
    /// let board = Board::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Right, 2), Some("abc".to_string()));
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Down, 2), Some("adg".to_string()));
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Left, 2), None);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Up, 2), None);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::UpRight, 2), None);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::UpLeft, 2), None);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::DownRight, 2), Some("aei".to_string()));
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::DownLeft, 2), None);
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Right, 0), Some("a".to_string()));
    /// assert_eq!(board.get_string_from_direction(0, 0, &Direction::Down, 0), Some("a".to_string()));
    ///
    pub fn get_string_from_direction(
        &self,
        start_x: usize,
        start_y: usize,
        direction: &Direction,
        distance: i32,
    ) -> Option<String> {
        // Get sequence of letters in the board, from a given position and direction.
        let mut seq = String::new();
        let coord_diff: CoordDiff = direction.to_coord_diff();
        for i in 0..distance + 1 {
            if let Some(s) = self.get_letter(
                Board::add(start_x, coord_diff.0 * i),
                Board::add(start_y, coord_diff.1 * i),
            ) {
                seq.push_str(&s);
            } else {
                return None;
            }
        }
        return Some(seq);
    }

    fn add(u: usize, i: i32) -> Option<usize> {
        // Prevent usize index overflow and handle substraction
        if i.is_negative() {
            u.checked_sub(i.wrapping_abs() as u32 as usize)
        } else {
            u.checked_add(i as usize)
        }
    }
}
#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpRight,
    UpLeft,
    DownRight,
    DownLeft,
}

pub struct CoordDiff(pub i32, pub i32);

impl Direction {
    pub fn to_coord_diff(&self) -> CoordDiff {
        match self {
            Direction::Up => CoordDiff(-1, 0),
            Direction::Down => CoordDiff(1, 0),
            Direction::Left => CoordDiff(0, -1),
            Direction::Right => CoordDiff(0, 1),
            Direction::UpRight => CoordDiff(-1, 1),
            Direction::UpLeft => CoordDiff(-1, -1),
            Direction::DownRight => CoordDiff(1, 1),
            Direction::DownLeft => CoordDiff(1, -1),
        }
    }
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
            Direction::UpRight,
            Direction::UpLeft,
            Direction::DownLeft,
            Direction::DownRight,
        ];
        DIRECTIONS.iter()
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    #[test]
    fn test_get_letter() {
        let b = Board::new(&vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(b.get_letter(Some(0), Some(0)), Some("a".to_string()));
        assert_eq!(b.get_letter(Some(0), Some(1)), Some("b".to_string()));
        assert_eq!(b.get_letter(Some(0), Some(2)), Some("c".to_string()));
    }
    #[test]
    fn test_get_string_from_direction() {
        let b = Board::new(&vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(
            b.get_string_from_direction(0, 0, &Direction::Right, 3),
            Some("abc".to_string())
        );
        assert_eq!(b.get_string_from_direction(0, 0, &Direction::Up, 3), None);
    }
    #[test]
    fn test_add() {
        assert_eq!(Board::add(0, 1), Some(1));
        assert_eq!(Board::add(0, -1), None);
        assert_eq!(Board::add(2, -1), Some(1));
    }
    #[test]
    fn test_get_all_possible_words() {
        use crate::trie::Trie;
        // 5x5 grid
        let b = Board::new(&vec![
            vec!['a', 'b', 'c', 'd', 'e'],
            vec!['f', 'g', 'h', 'i', 'j'],
            vec!['k', 'l', 'm', 'n', 'o'],
            vec!['p', 'q', 'r', 's', 't'],
            vec!['u', 'v', 'w', 'x', 'y'],
        ]);

        let words = vec!["nsx", "nrv", "nid", "nmlk"];
        let mut trie = Trie::new();
        trie.insert_words(&words);
        let mut result = Vec::new();
        let mut result_idx = Vec::new();
        b.get_all_possible_word(2, 3, &mut result, &mut result_idx, &b, &trie);
        assert!(result.len() == 4);
        assert!(result.contains(&"nsx".to_string()));
        assert!(result.contains(&"nrv".to_string()));
        assert!(result.contains(&"nid".to_string()));
        assert!(result.contains(&"nmlk".to_string()));
        println!("{:?}", result_idx);
        assert!(result_idx.len() == 4);
        assert!(result_idx.contains(&(13, 23)));
        assert!(result_idx.contains(&(13, 21)));
        assert!(result_idx.contains(&(13, 3)));
        assert!(result_idx.contains(&(13, 10)));
    }
}
