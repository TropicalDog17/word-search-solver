# word-search-solver

## Install dependencies

  sudo apt install libasound2-dev libudev-dev pkg-config
  
## Run the game

  ```cargo run --release```
  
## Run tests for library

  ```cargo test --release```
  
## Change config
Modify these parameters
``` 
pub const START_X: f32 = <X POSITION OF THE BOARD TOP-LEFT, in pixel>;
pub const GRID_SIZE: f32 = <SIZE OF EACH CELL IN GRID, in pixel>;
pub const START_Y: f32 = <Y POSITION OF THE BOARD TOP-LEFT, in pixel>;
pub const BOARD_SIZE: usize = 15; // number of cells in the grid, should left untouched to match the input file.
pub const SCREEN_WIDTH: f32 = ...;
pub const SCREEN_HEIGHT: f32 = ...;
```
