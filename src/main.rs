use ggez::conf::WindowMode;
use ggez::event::{self};
use ggez::{ContextBuilder, GameResult};
use std::{env, path};
use word_search_solver::constant::*;
use word_search_solver::state::main_state::MainState;
fn main() -> GameResult {
    // Make a Context.
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "Cool Game Author")
        .window_mode(window_state_mode())
        .add_resource_path(resource_dir)
        .build()
        .expect("aieee, could not create ggez context!");
    ctx.gfx.set_window_title("Word Search Visualizer");
    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let state = MainState::new(&mut ctx)?;

    // Run!
    event::run(ctx, event_loop, state);
}
pub fn window_state_mode() -> WindowMode {
    WindowMode {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
        maximized: false,
        fullscreen_type: ggez::conf::FullscreenType::Windowed,
        borderless: false,
        min_width: 800.,
        min_height: 600.,
        max_width: 1600.,
        max_height: 1200.,
        resizable: true,
        logical_size: None,
        ..Default::default()
    }
}
