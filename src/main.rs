pub mod entities;
pub mod updatable;
pub mod gamestate;

use tetra::ContextBuilder;
use crate::gamestate::GameState;

const WINDOW_WIDTH: i32 = 640;
const WINDOW_HEIGHT: i32 = 480;

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .show_mouse(false)
        .build()?
        .run(|ctx| GameState::new(ctx, "resources/"))
}
