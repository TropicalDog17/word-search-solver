use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Canvas, Color};
use ggez::{glam::*, Context, ContextBuilder, GameResult};
use std::path::Path;
use word_search_solver::utils::{fetch_board, fetch_target_words};
const SCREEN_WIDTH: f32 = 1024.0;
const SCREEN_HEIGHT: f32 = 768.0;
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

struct MainState {
    // Your state here...
    meshes: graphics::Mesh,
    letters: Vec<Vec<char>>,
    target_words: Vec<String>,
}

const GRID_SIZE: f32 = 30.0;
const START_X: f32 = 100.0;
const START_Y: f32 = 100.0;

impl MainState {
    pub fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let mb = &mut graphics::MeshBuilder::new();
        for i in 0..15 {
            // Horizontal lines
            mb.line(
                &[
                    Vec2::new(START_X, START_Y + GRID_SIZE * i as f32),
                    Vec2::new(START_X + 15.0 * GRID_SIZE, START_Y + GRID_SIZE * i as f32),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
            // Vertical lines
            mb.line(
                &[
                    Vec2::new(START_X + GRID_SIZE * i as f32, START_Y),
                    Vec2::new(START_X + GRID_SIZE * i as f32, START_Y + 15.0 * GRID_SIZE),
                ],
                1.0,
                Color::new(0.0, 0.0, 0.0, 1.0),
            )?;
        }
        mb.rectangle(
            graphics::DrawMode::stroke(1.0),
            graphics::Rect::new(START_X, START_Y, 15.0 * GRID_SIZE, 15.0 * GRID_SIZE),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;

        // Load/create resources such as images here.
        let meshes = graphics::Mesh::from_data(_ctx, mb.build());
        // Load board and target words
        let board_file_path = Path::new("src/input/board.txt");
        let target_words_file_path = Path::new("src/input/words.txt");
        let target_words = fetch_target_words(target_words_file_path);
        let letters: Vec<Vec<char>> = fetch_board(board_file_path);

        let s = MainState {
            // ...
            meshes,
            letters,
            target_words,
        };
        Ok(s)
    }
}
fn build_grid_mesh(ctx: &mut Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    for i in 0..15 {
        // Horizontal lines
        mb.line(
            &[
                Vec2::new(START_X, START_Y + GRID_SIZE * i as f32),
                Vec2::new(START_X + 15.0 * GRID_SIZE, START_Y + GRID_SIZE * i as f32),
            ],
            1.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
        // Vertical lines
        mb.line(
            &[
                Vec2::new(START_X + GRID_SIZE * i as f32, START_Y),
                Vec2::new(START_X + GRID_SIZE * i as f32, START_Y + 15.0 * GRID_SIZE),
            ],
            1.0,
            Color::new(0.0, 0.0, 0.0, 1.0),
        )
        .unwrap();
    }
    mb.rectangle(
        graphics::DrawMode::stroke(1.0),
        graphics::Rect::new(START_X, START_Y, 15.0 * GRID_SIZE, 15.0 * GRID_SIZE),
        graphics::Color::new(1.0, 0.0, 0.0, 1.0),
    )
    .unwrap();
    graphics::Mesh::from_data(ctx, mb.build())
}
fn draw_strike_through(ctx: &mut Context, start_idx: u32, end_idx: u32, canvas: &mut Canvas) {
    let start = Vec2::new(
        START_X + GRID_SIZE * (start_idx % 15) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (start_idx / 15) as f32 + GRID_SIZE / 2.0,
    );
    let end = Vec2::new(
        START_X + GRID_SIZE * (end_idx % 15) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (end_idx / 15) as f32 + GRID_SIZE / 2.0,
    );

    // Draw strike through
    let mb = &mut graphics::MeshBuilder::new();
    mb.line(&[start, end], 1.0, Color::new(0.0, 1.0, 0.0, 1.0))
        .unwrap();
    let strike_through_mesh = graphics::Mesh::from_data(ctx, mb.build());
    canvas.draw(&strike_through_mesh, graphics::DrawParam::new());
}
impl EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // Update code here...
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // Draw code here...
        canvas.draw(&self.meshes, graphics::DrawParam::new());
        for i in 0..15 {
            for j in 0..15 {
                let text_dest = graphics::DrawParam::new()
                    .dest(Vec2::new(
                        START_X + GRID_SIZE * i as f32 + GRID_SIZE / 2.0,
                        START_Y + GRID_SIZE * j as f32 + GRID_SIZE / 2.0,
                    ))
                    .offset(Vec2::new(0.5, 0.5));
                canvas.draw(
                    graphics::Text::new(self.letters[i][j]).set_scale(20.),
                    text_dest,
                );
            }
        }
        draw_strike_through(ctx, 0, 14, &mut canvas);
        canvas.finish(ctx)?;
        Ok(())
    }
}
