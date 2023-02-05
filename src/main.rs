mod graph;
mod edit_state;

use macroquad::prelude::*;

use graph::*;
use edit_state::*;

#[macroquad::main("Graphite")]
async fn main() {
    let mut graph = Graph::new();
    let mut edit = EditState::None(0);
    // cursor on mouse down to distinguish between drag and click
    let mut cursor0: Option<(f32, f32)> = None;
    let mut mouse_dragging = false;

    loop {
        clear_background(WHITE);
        if is_mouse_button_down(MouseButton::Left) {
            if let Some(c) = cursor0 {
                if c != mouse_position() {
                    mouse_dragging = true;
                }
            } else {
                cursor0 = Some(mouse_position());
            }
        } else if cursor0.is_some() {
            if mouse_dragging {
                mouse_dragging = false;
                edit = edit.left_mouse_drag_stop();
            } else {
                edit = edit.left_mouse_clicked(mouse_position(), &mut graph);
            }
            cursor0 = None;
        }
        if mouse_dragging {
            edit = edit.left_mouse_dragging(mouse_position(), &mut graph);
        }
        graph.render(edit.selected());
        next_frame().await;
    }
}

