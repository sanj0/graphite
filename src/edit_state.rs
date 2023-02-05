use crate::graph::*;

use macroquad::prelude::Vec2;

pub enum EditState {
    None(u32),
    Selected {node: usize, iota: u32},
    Dragging {node: usize, iota: u32},
}

impl EditState {
    pub fn selected(&self) -> Option<usize>{
        match self {
            Self::Selected{node, ..} => Some(*node),
            Self::Dragging{node, ..} => Some(*node),
            _ => None,
        }
    }

    /// Called on every frame the mouse is dragging to operate the editing of the graph and
    /// changing the state machine
    pub fn left_mouse_dragging(self, cursor: (f32, f32), graph: &mut Graph) -> Self {
        match self {
            Self::None(iota) => {
                if let Some(node) = graph.node_under_cursor(cursor) {
                    Self::Dragging{node, iota}
                } else {
                    Self::None(iota)
                }
            }
            Self::Dragging{node, iota} => {
                graph.nodes[node].pos = Vec2::new(cursor.0, cursor.1);
                Self::Dragging{node, iota}
            }
            Self::Selected{node, iota} => {
                Self::Selected{node, iota}
            }
        }
    }

    /// Called on every frame the mouse has stopped dragging (which is usually no more than once in
    /// a row) to operate the editing of the graph and changing the state machine
    pub fn left_mouse_drag_stop(self) -> Self {
        match self {
            // shouldn't happen with natural inputs
            Self::None(iota) => Self::None(iota),
            Self::Dragging{node: _, iota} => Self::None(iota),
            // shouldn't happen with natural inputs
            Self::Selected{node: _, iota} => Self::None(iota),
        }
    }

    /// Called on every frame the mouse has clicked (which is usually no more than once in
    /// a row) to operate the editing of the graph and changing the state machine
    pub fn left_mouse_clicked(self, cursor: (f32, f32), graph: &mut Graph) -> Self {
        let node_index = graph.node_under_cursor(cursor);
        match self {
            Self::None(iota) => {
                if let Some(index) = node_index {
                    Self::Selected{node: index, iota}
                } else {
                    let node = Node::new(Vec2::new(cursor.0, cursor.1),
                        gen_label(LabelType::Alpha, iota).expect("label gen err"));
                    graph.nodes.push(node);
                    Self::None(iota + 1)
                }
            }
            Self::Selected{node, iota} => {
                if let Some(index) = node_index {
                    if index != node {
                        let link = Link::new(node, index);
                        if graph.links.contains(&link) {
                            graph.links.retain(|&l| l != link);
                        } else {
                            graph.links.push(link);
                        }
                    }
                }
                Self::None(iota)
            }
            // shouldn't really be able to happen
            Self::Dragging{node: _, iota} => Self::None(iota),
        }
    }
}

