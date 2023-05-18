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
            letters: letters.to_vec(),
            cols: cols,
            rows: rows,
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
