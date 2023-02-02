mod graph;
mod edit_state;

use macroquad::prelude::*;

use graph::*;
use edit_state::*;

#[macroquad::main("Graphite")]
async fn main() {
    let mut graph = Graph::new();
    let mut edit = EditState::None(0);

    loop {
        clear_background(WHITE);
        if is_mouse_button_pressed(MouseButton::Left) {
            edit = edit.left_mouse_pressed(mouse_position(), &mut graph);
        }
        graph.render(edit.selected());
        next_frame().await;
    }
}

