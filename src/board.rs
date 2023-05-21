use crate::trie::Trie;
use std::slice::Iter;
pub struct Board {
    letters: Vec<Vec<char>>,
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
    /// board.get_all_possible_word(0, 0, &mut result, &board, &trie);
    /// let expected_result: HashSet<&str> = HashSet::from(["abc", "adg"]);
    /// assert!(expected_result.contains(result[0].as_str()));
    /// assert!(expected_result.contains(result[1].as_str()));
    /// ```
    ///
    ///

    pub fn get_all_possible_word(
        &self,
        i: usize,
        j: usize,
        result: &mut Vec<String>,
        board: &Board,
        trie: &Trie,
    ) {
        // Check if a given position is valid
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
    pub fn get_rows(&self) -> usize {
        self.rows
    }
    pub fn get_cols(&self) -> usize {
        self.cols
    }
    pub fn get_letter(&self, x: Option<usize>, y: Option<usize>) -> Option<String> {
        if x == None || y == None {
            return None;
        }

        let letter = self.letters.get(x.unwrap())?.get(y.unwrap());
        if let Some(letter) = letter {
            Some(String::from(*letter))
        } else {
            None
        }
    }
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
        if coord_diff.0 < 0 || coord_diff.1 < 0 {}
        for i in 0..distance {
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
    use super::*;
    #[test]
    fn test_get_letter() {
        let b = Board::new(&vec![
            vec!['a', 'b', 'c'],
            vec!['d', 'e', 'f'],
            vec!['g', 'h', 'i'],
        ]);
        assert_eq!(b.get_letter(Some(0), Some(0)), Some("a".to_string()));
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
}
