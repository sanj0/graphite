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

    pub fn left_mouse_pressed(&self, cursor: (f32, f32), graph: &mut Graph) -> Self {
        let node_index = graph.node_under_cursor(cursor);
        match self {
            Self::None(iota) => {
                let iota = *iota;
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
                    if index != *node {
                        let link = Link::new(*node, index);
                        if graph.links.contains(&link) {
                            graph.links.retain(|&l| l != link);
                        } else {
                            graph.links.push(link);
                        }
                    }
                }
                Self::None(*iota)
            }
            // shouldn't really be able to happen
            Self::Dragging{node: _, iota} => Self::None(*iota),
        }
    }
}

