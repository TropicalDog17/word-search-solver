use crate::constant::*;
use crate::state::main_state::MainState;
use ggez::graphics::{self, Canvas, Color, Rect};
use ggez::{glam::*, Context, GameResult};
use std::fs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
pub fn fetch_board(file_path: &Path) -> Vec<Vec<char>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut result = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut vec_letter = Vec::new();
            for c in line.chars() {
                if c.is_alphabetic() {
                    vec_letter.push(c);
                }
            }
            result.push(vec_letter.clone());
        }
    }

    println!("Letters: {:?}", result);
    result
}

pub fn fetch_target_words(file_path: &Path) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut result = Vec::new();
    for word in contents.split(" ") {
        result.push(word.to_owned());
    }
    result
}
pub fn build_grid(ctx: &mut Context) -> graphics::Mesh {
    let mb = &mut graphics::MeshBuilder::new();
    // for i in 0..BOARD_SIZE {
    //     // Horizontal lines
    //     mb.line(
    //         &[
    //             Vec2::new(START_X, START_Y + GRID_SIZE * i as f32),
    //             Vec2::new(
    //                 START_X + GRID_SIZE * BOARD_SIZE as f32,
    //                 START_Y + GRID_SIZE * i as f32,
    //             ),
    //         ],
    //         1.0,
    //         Color::new(0.0, 0.0, 0.0, 1.0),
    //     )
    //     .unwrap();
    //     // Vertical lines
    //     mb.line(
    //         &[
    //             Vec2::new(START_X + GRID_SIZE * i as f32, START_Y),
    //             Vec2::new(
    //                 START_X + GRID_SIZE * i as f32,
    //                 START_Y + GRID_SIZE * BOARD_SIZE as f32,
    //             ),
    //         ],
    //         1.0,
    //         Color::new(0.0, 0.0, 0.0, 1.0),
    //     )
    //     .unwrap();
    // }
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
pub fn draw_highlighted_line(
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
            Color::new(1.0, 0.0, 0.0, 0.5),
        )
        .unwrap();
    } else {
        mb.line(&[start, end], 10.0, Color::new(0.0, 1.0, 0.0, 1.0))
            .unwrap();
    }
    // Draw strike through

    let line_meshes = graphics::Mesh::from_data(ctx, mb.build());
    canvas.draw(&line_meshes, graphics::DrawParam::default());
}
pub fn draw_highlight(
    ctx: &mut Context,
    start: Vec2,
    end: Vec2,
    canvas: &mut Canvas,
    feasible: bool,
) {
    if !feasible {
        return;
    }
    let start_x = START_X + GRID_SIZE * 0 as f32;
    let start_y = START_Y + GRID_SIZE * 0 as f32;

    let rect = Rect::new(start_x, start_y, GRID_SIZE * 15.0, GRID_SIZE * 1.0);
    let mb = &mut graphics::MeshBuilder::new();
    if start == end {
        mb.circle(
            graphics::DrawMode::fill(),
            start,
            GRID_SIZE / 2.0,
            1.0,
            Color::new(0.0, 0.0, 1.0, 1.0),
        )
        .unwrap();
    } else {
        mb.rounded_rectangle(graphics::DrawMode::fill(), rect, 15.0, Color::YELLOW)
            .unwrap();
    }
    // Draw strike through

    let line_meshes = graphics::Mesh::from_data(ctx, mb.build());
    canvas.draw(
        &line_meshes,
        graphics::DrawParam::default()
            .offset([0.0, 0.0])
            .rotation(1.0), // .rotation(1.0),
    );
} // Draw strike through that stay on the screen
pub fn draw_line(
    state: &mut MainState,
    ctx: &mut Context,
    start_idx: usize,
    end_idx: usize,
    mb: &mut graphics::MeshBuilder,
    canvas: &mut Canvas,
) -> GameResult<()> {
    let start = Vec2::new(
        START_X + GRID_SIZE * (start_idx % BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (start_idx / BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
    );
    let end = Vec2::new(
        START_X + GRID_SIZE * (end_idx % BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
        START_Y + GRID_SIZE * (end_idx / BOARD_SIZE) as f32 + GRID_SIZE / 2.0,
    );
    mb.line(&[start, end], 5.0, Color::new(0.0, 0.0, 1.0, 1.0))?;
    // Draw strike through
    let line_mesh = graphics::Mesh::from_data(ctx, mb.build());
    state.line_mesh = line_mesh;
    canvas.draw(&state.line_mesh, graphics::DrawParam::new());
    Ok(())
}
// display out answer as we find words
pub fn display_word(ctx: &mut Context, canvas: &mut Canvas, word: &str, idx: usize) {
    let draw_params = graphics::DrawParam::new()
        .dest([
            START_X + GRID_SIZE * BOARD_SIZE as f32 + 100.0,
            START_Y + 50.0 * idx as f32,
        ])
        .color(Color::new(0.0, 0.0, 0.0, 1.0));
    canvas.draw(
        graphics::Text::new(word)
            .set_scale(30.)
            .set_font("Montserrat"),
        draw_params,
    );
}
#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::utils::fetch_board;
    #[test]
    fn ensure_board_input_exists() {
        let file_path = Path::new("src/input/board.txt");
        fetch_board(file_path);
    }
    #[test]
    fn ensure_target_words_input_exists() {
        let file_path = Path::new("src/input/words.txt");
        fetch_board(file_path);
    }
}
