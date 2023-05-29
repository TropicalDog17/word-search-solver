use crate::constant::*;
use crate::state::board_state::Board;
use crate::state::search_state::SearchState;
use crate::trie::Trie;
use crate::utils::*;
use ggez::event::EventHandler;
use ggez::graphics::{self};
use ggez::{glam::*, Context, GameResult};
use std::path::Path;
pub struct MainState {
    pub grid_mesh: graphics::Mesh,
    pub line_mesh: graphics::Mesh,
    pub board_state: Board,
    pub mb: graphics::MeshBuilder,
    pub trie: Trie,
    pub found_words_idx: Vec<(usize, usize)>,
    pub current_idx: (Vec2, Vec2), // Current line position to check if it is a word
    pub search_state: SearchState,
}
impl MainState {
    pub fn new(ctx: &mut Context) -> GameResult<MainState> {
        // Load board and target words
        let board_file_path = Path::new("src/input/board_months.txt");
        let target_words_file_path = Path::new("src/input/months.txt");
        let letters: Vec<Vec<char>> = fetch_board(board_file_path);
        if letters.len() != BOARD_SIZE || letters[0].len() != BOARD_SIZE {
            panic!("Board size is not correct, please check the input file or modify the BOARD_SIZE constant");
        }
        // Initialize grid
        let grid_mesh = build_grid(ctx);

        // Initialize mesh & mesh builder, for building persistent lines
        let mb = &mut graphics::MeshBuilder::new();
        let line_mesh = graphics::Mesh::from_data(ctx, mb.build());

        let board_state = Board::new(&letters);

        // Initialize trie
        let target_words: Vec<String> = fetch_target_words(target_words_file_path);
        let target_words_str = target_words.iter().map(String::as_str).collect();
        let trie = Trie::from(&target_words_str);
        let s = MainState {
            // ...
            grid_mesh,
            line_mesh,
            board_state,
            mb: graphics::MeshBuilder::new(),
            trie,
            found_words_idx: Vec::new(),
            current_idx: (Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)),
            search_state: SearchState::new(),
        };
        ctx.gfx.add_font(
            "Montserrat",
            graphics::FontData::from_path(ctx, "/Montserrat-Bold.ttf")?,
        );
        Ok(s)
    }
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
            graphics::Canvas::from_frame(ctx, graphics::Color::from([1.0, 1.0, 1.0, 1.0]));
        // Draw code here...
        canvas.draw(&self.grid_mesh, graphics::DrawParam::new());
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                let text_dest = graphics::DrawParam::new()
                    .dest(Vec2::new(
                        START_X + GRID_SIZE * j as f32 + GRID_SIZE / 2.0,
                        START_Y + GRID_SIZE * i as f32 + GRID_SIZE / 2.0,
                    ))
                    .color(graphics::Color::from([0.0, 0.0, 0.0, 1.0]))
                    .offset(Vec2::new(0.5, 0.5));
                canvas.draw(
                    graphics::Text::new(self.board_state.letters[i][j])
                        .set_scale(30.)
                        .set_font("Montserrat"),
                    text_dest,
                );
            }
        }
        let mut mb = self.mb.clone();
        // Construct the line mesh based on the found words
        let fw = self.found_words_idx.clone();
        for (idx, st) in fw.iter().enumerate() {
            let start_idx = st.0;
            let end_idx = st.1;
            draw_line(self, ctx, start_idx, end_idx, &mut mb, &mut canvas)?;
            let word = self
                .board_state
                .get_word_from_1d_position(start_idx, end_idx);
            display_word(ctx, &mut canvas, &word, idx)
        }
        // Draw the currently checking line
        draw_highlighted_line(
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
