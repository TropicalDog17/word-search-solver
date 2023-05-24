use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::path::Path;
use word_search_solver::board::{Board, SearchState};
use word_search_solver::constant::*;
use word_search_solver::trie::Trie;
use word_search_solver::utils::{fetch_board, fetch_target_words};
fn main() -> GameResult {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()
        .expect("aieee, could not create ggez context!");
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = MainState::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, state);
}

pub struct MainState {
    // Your state here...
    meshes: graphics::Mesh,
    strike_through_meshes: graphics::Mesh,
    board_state: Board,
    mb: graphics::MeshBuilder,
    trie: Trie,
    found_words_idx: Vec<(usize, usize)>,
    current_idx: (Vec2, Vec2), // Current line position to check if it is a word
    search_state: SearchState,
}

const GRID_SIZE: f32 = 30.0;
const START_X: f32 = 100.0;
const START_Y: f32 = 100.0;
const BOARD_SIZE: usize = 15; // grid of square
impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        // Load board and target words
        let board_file_path = Path::new("src/input/board.txt");
        let target_words_file_path = Path::new("src/input/words.txt");
        let target_words = fetch_target_words(target_words_file_path);
        let letters: Vec<Vec<char>> = fetch_board(board_file_path);
        if letters.len() != BOARD_SIZE || letters[0].len() != BOARD_SIZE {
            panic!("Board size is not correct, please check the input file or modify the BOARD_SIZE constant");
        }
        let mb = &mut graphics::MeshBuilder::new();
        for i in 0..BOARD_SIZE {
            // Horizontal lines
            mb.line(
                &[
                    Vec2::new(START_X, START_Y + GRID_SIZE * i as f32),
                    Vec2::new(
                        START_X + GRID_SIZE * BOARD_SIZE as f32,
                        START_Y + GRID_SIZE * i as f32,
                    ),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            // Vertical lines
            mb.line(
                &[
                    Vec2::new(START_X + GRID_SIZE * i as f32, START_Y),
                    Vec2::new(
                        START_X + GRID_SIZE * i as f32,
                        START_Y + GRID_SIZE * BOARD_SIZE as f32,
                    ),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
        }
        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(
                START_X,
                START_Y,
                GRID_SIZE * BOARD_SIZE as f32,
                GRID_SIZE * BOARD_SIZE as f32,
            ),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        // Load/create resources such as images here.
        let meshes = graphics::Mesh::from_data(_ctx, mb.build());

        // Initialize empty mesh strikethough
        let mb = &mut graphics::MeshBuilder::new();
        let strike_through_meshes = graphics::Mesh::from_data(_ctx, mb.build());
        let board_state = Board::new(&letters);
        let mut trie = Trie::new();
        // Vec<String> to Vec<&str>
        let target_words_str = target_words.iter().map(String::as_str).collect();
        trie.insert_words(&target_words_str);
        let found_words_idx = Vec::new();
        let s = MainState {
            // ...
            meshes,
            strike_through_meshes,
            board_state,
            mb: graphics::MeshBuilder::new(),
            trie,
            found_words_idx,
            current_idx: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            search_state: SearchState::new(),
        };
        Ok(s)
    }
}
fn build_grid_mesh(ctx: &mut Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    for i in 0..BOARD_SIZE {
        // Horizontal lines
        mb.line(
            &[
                Vec2::new(START_X, START_Y + GRID_SIZE * i as f32),
                Vec2::new(
                    START_X + GRID_SIZE * BOARD_SIZE as f32,
                    START_Y + GRID_SIZE * i as f32,
                ),
            ],
            1.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
        // Vertical lines
        mb.line(
            &[
                Vec2::new(START_X + GRID_SIZE * i as f32, START_Y),
                Vec2::new(
                    START_X + GRID_SIZE * i as f32,
                    START_Y + GRID_SIZE * BOARD_SIZE as f32,
                ),
            ],
            1.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
    }
    mb.rectangle(
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(
            START_X,
            START_Y,
            GRID_SIZE * BOARD_SIZE as f32,
            GRID_SIZE * BOARD_SIZE as f32,
        ),
        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
    )
    .unwrap();
    graphics::Mesh::from_data(ctx, mb.build())
}

// Draw temporary strike through that disappear in the next frame
fn draw_temp_strike_through(
    ctx: &mut Context,
    start: Vec2,
    end: Vec2,
    canvas: &mut Canvas,
    feasible: bool,
) {
    if !feasible {
        return;
    }
    let start = Vec2::new(
        START_X + GRID_SIZE * start.x as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * start.y as f32 + GRID_SIZE / 2.0,
    );
    let end = Vec2::new(
        START_X + GRID_SIZE * end.x as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * end.y as f32 + GRID_SIZE / 2.0,
    );
    let mb = &mut graphics::MeshBuilder::new();
    if start == end {
        mb.circle(
            graphics::DrawMode::fill(),
            start,
            GRID_SIZE / 2.0,
            1.0,
            Color::new(1.0, 1.0, 1.0, 1.0),
        )
        .unwrap();
    } else {
        mb.line(&[start, end], 5.0, Color::new(1.0, 1.0, 0.0, 1.0))
            .unwrap();
    }
    // Draw strike through

    let strike_through_meshes = graphics::Mesh::from_data(ctx, mb.build());
    canvas.draw(&strike_through_meshes, graphics::DrawParam::default());
}
// Draw strike through that stay on the screen
fn draw_strike_through(
    state: &mut MainState,
    ctx: &mut Context,
    start_idx: usize,
    end_idx: usize,
    mb: &mut graphics::MeshBuilder,
    canvas: &mut Canvas,
) {
    let start = Vec2::new(
        START_X + GRID_SIZE * (start_idx % BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (start_idx / BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
    );
    let end = Vec2::new(
        START_X + GRID_SIZE * (end_idx % BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (end_idx / BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
    );
    // TODO: Refactor unsafe unwrap here
    mb.line(&[start, end], 2.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .unwrap();
    // Draw strike through
    let strike_through_mesh = graphics::Mesh::from_data(ctx, mb.build());
    state.strike_through_meshes = strike_through_mesh;
    canvas.draw(&state.strike_through_meshes, graphics::DrawParam::new());
}
impl EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // Update code here...
        const DESIRED_FPS: u32 = 30;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if let Some(pos) = self.search_state.current_prefix() {
                self.current_idx = pos.to_vec2();
            }
            if let Some(word_position) = self
                .board_state
                .check_state(&mut self.search_state, &self.trie)
            {
                println!("Found word: {:?}", self.search_state.current_prefix());
                self.found_words_idx.push(word_position.to_1d(BOARD_SIZE));
            }
            match self
                .board_state
                .next_state(&self.search_state, self.search_state.feasible)
            {
                Some(state) => {
                    self.search_state = state;
                    println!("{:?}", self.search_state);
                }
                None => {
                    // sleep

                    std::thread::sleep(std::time::Duration::from_secs(20));
                    ctx.request_quit();
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // Draw code here...
        canvas.draw(&self.meshes, graphics::DrawParam::new());
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let text_dest = graphics::DrawParam::new()
                    .dest(Vec2::new(
                        START_X + GRID_SIZE * j as f32 + GRID_SIZE / 2.0,
                        START_Y + GRID_SIZE * i as f32 + GRID_SIZE / 2.0,
                    ))
                    .offset(Vec2::new(0.5, 0.5));
                canvas.draw(
                    graphics::Text::new(self.board_state.letters[i][j]).set_scale(20.),
                    text_dest,
                );
            }
        }
        let mut mb = self.mb.clone();
        // Construct the strike through mesh based on the found words
        let fw = self.found_words_idx.clone();
        for st in fw.iter() {
            let start_idx = st.0;
            let end_idx = st.1;
            draw_strike_through(self, ctx, start_idx, end_idx, &mut mb, &mut canvas);
        }
        draw_temp_strike_through(
            ctx,
            self.current_idx.0,
            self.current_idx.1,
            &mut canvas,
            self.search_state.feasible,
        );
        self.mb = mb;
        canvas.finish(ctx)?;
        Ok(())
    }
}
