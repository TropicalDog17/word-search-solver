use crate::board::*;
use crate::constant::*;
#[derive(Debug, PartialEq, Eq)]
pub struct SearchState {
    pub position: (usize, usize),
    pub direction: Direction,
    pub distance: i32,
    pub limit: usize,
    pub feasible: bool,
}
impl Default for SearchState {
    fn default() -> Self {
        SearchState {
            position: (0, 0),
            direction: Direction::Up,
            distance: 0,
            limit: BOARD_SIZE,
            feasible: true,
        }
    }
}
impl SearchState {
    pub fn new() -> Self {
        SearchState::default()
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
        Board::get_pos_from_direction(
            self.position.0,
            self.position.1,
            &self.direction,
            self.distance,
        )
        .map(|end| WordPosition::new(start, end))
    }
}
