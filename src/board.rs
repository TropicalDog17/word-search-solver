use crate::constant::BOARD_SIZE;
use crate::trie::Trie;
use ggez::glam::Vec2;
use std::slice::Iter;
pub struct Board {
    pub letters: Vec<Vec<char>>,
    cols: usize,
    rows: usize,
}
#[derive(Debug)]

pub struct WordPosition {
    start: (usize, usize),
    end: (usize, usize),
}
impl WordPosition {
    pub fn new(start: (usize, usize), end: (usize, usize)) -> Self {
        WordPosition { start, end }
    }
    ///
    pub fn to_1d(&self, board_size: usize) -> (usize, usize) {
        let (start_i, start_j) = self.start;
        let (end_i, end_j) = self.end;
        let start = start_i * board_size + start_j;
        let end = end_i * board_size + end_j;
        (start, end)
    }
    /// Convert the raw usize position to a Vec2 tuple
    pub fn to_vec2(&self) -> (Vec2, Vec2) {
        let start = Vec2::new(self.start.1 as f32, self.start.0 as f32);
        let end = Vec2::new(self.end.1 as f32, self.end.0 as f32);
        (start, end)
    }
}
#[derive(Debug)]
pub struct SearchState {
    pub position: (usize, usize),
    pub direction: Direction,
    pub distance: i32,
    pub limit: usize,
    pub feasible: bool,
}
impl SearchState {
    pub fn new() -> Self {
        SearchState {
            position: (0, 0),
            direction: Direction::Up,
            distance: 0,
            limit: BOARD_SIZE,
            feasible: true,
        }
    }
    pub fn from(position: (usize, usize), direction: Direction, distance: i32) -> Self {
        SearchState {
            position,
            direction,
            distance,
            limit: BOARD_SIZE,
            feasible: true,
        }
    }
    pub fn current_prefix(&self) -> Option<WordPosition> {
        let start = self.position;
        if let Some(end) = Board::get_pos_from_direction(
            self.position.0,
            self.position.1,
            &self.direction,
            self.distance,
        ) {
            return Some(WordPosition::new(start, end));
        } else {
            None
        }
    }
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
    /// Given current position, return the next position in the board
    /// # Arguments
    /// * `i` - The row index of the position
    /// * `j` - The column index of the position
    /// # Returns
    /// * `Option<(usize, usize)>` - The next position
    /// # Example
    /// ```
    /// use word_search_solver::board::Board;
    /// let board = Board::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    /// let (i,j) = (0,0);
    /// let next_pos = board.next_pos(i,j);
    /// assert_eq!(next_pos, Some((0,1)));
    /// let (i, j) = (0, 2);
    /// let next_pos = board.next_pos(i,j);
    /// assert_eq!(next_pos, Some((1,0)));
    /// let (i, j) = (2, 2);
    /// let next_pos = board.next_pos(i,j);
    /// assert_eq!(next_pos, None);
    /// ```
    pub fn next_pos(&self, i: usize, j: usize) -> Option<(usize, usize)> {
        let mut i = i;
        let mut j = j;
        j += 1;
        if j == self.cols {
            j = 0;
            i += 1;
        }
        // Handle overflow
        if i == self.rows {
            None
        }
        // Return the next position
        else {
            Some((i, j))
        }
    }
    pub fn next_state(&self, state: &SearchState, feasible: bool) -> Option<SearchState> {
        let (i, j) = state.position;
        let distance = state.distance;
        let direction = state.direction;
        if let None = self.get_string_from_direction(i, j, &direction, distance) {
            if let None = direction.next() {
                if let None = self.next_pos(i, j) {
                    return None;
                } else {
                    return Some(SearchState::from(
                        self.next_pos(i, j).unwrap(),
                        Direction::Up,
                        0,
                    ));
                }
            }
            return Some(SearchState::from(
                state.position,
                direction.next().unwrap(),
                0,
            ));
        } else {
            if feasible {
                return Some(SearchState::from(
                    state.position,
                    state.direction,
                    state.distance + 1,
                ));
            } else {
                if let None = direction.next() {
                    if let None = self.next_pos(i, j) {
                        return None;
                    } else {
                        return Some(SearchState::from(
                            self.next_pos(i, j).unwrap(),
                            Direction::Up,
                            0,
                        ));
                    }
                }
                return Some(SearchState::from(
                    state.position,
                    direction.next().unwrap(),
                    0,
                ));
            }
        }
    }
    pub fn check_state(&self, state: &mut SearchState, trie: &Trie) -> Option<WordPosition> {
        let (i, j) = state.position;
        let distance = state.distance;
        let direction = state.direction;
        let string = self.get_string_from_direction(i, j, &direction, distance);
        if let None = string {
            return None;
        }
        let string = string.unwrap();
        if !trie.starts_with(&string) {
            state.feasible = false;
        } else {
            match self.get_string_from_direction(i, j, &direction, distance + 1) {
                None => state.feasible = false,
                Some(_) => state.feasible = true,
            }
            if trie.search(&string) {
                let word_position = WordPosition::new(
                    (i, j),
                    Board::get_pos_from_direction(i, j, &direction, distance).unwrap_or_default(),
                );
                return Some(word_position);
            }
        }
        None
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
    /// let mut current_idx: (usize, usize) = (0, 0);
    /// board.get_all_possible_word(0, 0, &mut result, &mut result_idx, &board, &trie, &mut current_idx );
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
        current_idx: &mut (usize, usize),
    ) {
        // Check if a given position is valid
        for d in Direction::iterator() {
            let mut dist: i32 = 0;
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
                    let position_idx = (
                        start.0 * board.get_cols() + start.1,
                        end.0 * board.get_cols() + end.1,
                    );
                    *current_idx = position_idx;
                    result.push(prefix.clone());
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
    ///
    /// Get the position in the board from a given position and direction
    /// # Arguments
    /// * `i` - The row index of the position
    /// * `j` - The column index of the position
    /// * `direction` - The direction to search
    /// * `distance` - The distance to search, if = 0 then return the letter at the position
    /// # Examples
    /// ```
    /// use word_search_solver::board::Board;
    /// use word_search_solver::board::Direction;
    /// let board = Board::new(&vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f'], vec!['g', 'h', 'i']]);
    /// assert_eq!(Board::get_pos_from_direction(0, 0, &Direction::Right, 2), Some((0, 2)));
    /// assert_eq!(Board::get_pos_from_direction(0, 0, &Direction::Down, 2), Some((2, 0)));
    /// assert_eq!(Board::get_pos_from_direction(0, 0, &Direction::Left, 2), None);
    ///
    pub fn get_pos_from_direction(
        i: usize,
        j: usize,
        direction: &Direction,
        distance: i32,
    ) -> Option<(usize, usize)> {
        let coord_diff: CoordDiff = direction.to_coord_diff();
        let x = Board::add(i, coord_diff.0 * distance);
        let y = Board::add(j, coord_diff.1 * distance);
        if x == None || y == None {
            return None;
        }
        Some((x.unwrap(), y.unwrap()))
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

#[derive(Debug, Clone, Copy)]
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
    pub fn next(&self) -> Option<Direction> {
        match self {
            Direction::Up => Some(Direction::Down),
            Direction::Down => Some(Direction::Left),
            Direction::Left => Some(Direction::Right),
            Direction::Right => Some(Direction::UpRight),
            Direction::UpRight => Some(Direction::UpLeft),
            Direction::UpLeft => Some(Direction::DownLeft),
            Direction::DownLeft => Some(Direction::DownRight),
            Direction::DownRight => None,
        }
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
            b.get_string_from_direction(0, 0, &Direction::Right, 2),
            Some("abc".to_string())
        );
        assert_eq!(
            b.get_string_from_direction(0, 0, &Direction::Down, 2),
            Some("adg".to_string())
        );
        assert_eq!(
            b.get_string_from_direction(0, 0, &Direction::DownRight, 2),
            Some("aei".to_string())
        );
        assert_eq!(b.get_string_from_direction(0, 0, &Direction::Left, 2), None);
        assert_eq!(b.get_string_from_direction(0, 0, &Direction::Up, 1), None);
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
        let mut current_idx = (0, 0);
        b.get_all_possible_word(
            2,
            3,
            &mut result,
            &mut result_idx,
            &b,
            &trie,
            &mut current_idx,
        );
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
