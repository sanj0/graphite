use macroquad::prelude::*;

pub struct Graph {
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
}

pub struct Node {
    pub pos: Vec2,
    pub label: char,
}

/// Strores indexes of a and b node
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Link {
    pub a: usize,
    pub b: usize,
}

pub enum LabelType {
    Alpha,
    Num,
    Rom,
}

impl Node {
    pub fn new(pos: Vec2, label: char) -> Self {
        Self {
            pos,
            label,
        }
    }
}

impl Link {
    pub fn new(a: usize, b: usize) -> Self {
        Self {a, b}
    }
}

impl Graph {
    pub const NODE_SIZE: f32 = 25.;
    pub const NODE_LABEL_FONT_SIZE: f32 = 30.;
    pub const LINK_THICKNESS: f32 = 4.;

    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            links: Vec::new(),
        }
    }

    pub fn render(&self, selected: Option<usize>) {
        macro_rules! render_node {
            ($n: ident, $c: ident) => {
                let x = $n.pos.x;
                let y = $n.pos.y;
                draw_circle(x, y, Self::NODE_SIZE, $c);
                draw_text(&$n.label.to_string(), x, y, Self::NODE_LABEL_FONT_SIZE, WHITE);
            }
        }
        for l in &self.links {
            let a = self.nodes[l.a].pos;
            let b = self.nodes[l.b].pos;
            draw_line(a.x, a.y, b.x, b.y, Self::LINK_THICKNESS, BLACK);
        }
        for n in &self.nodes {
            render_node!(n, DARKBLUE);
        }
        // just draw selected node twice; much faster than checking index in render loop above
        if let Some(index) = selected {
            let node = &self.nodes[index];
            render_node!(node, RED);
        }
    }

    pub fn node_under_cursor(&self, cursor: (f32, f32)) -> Option<usize> {
        for (i, n) in self.nodes.iter().enumerate() {
            let x = n.pos.x;
            let y = n.pos.y;
            let d = ((x - cursor.0).powi(2) + (y - cursor.1).powi(2)).sqrt();
            if d <= Self::NODE_SIZE {
                return Some(i);
            }
        }
        None
    }
}

pub fn gen_label(ty: LabelType, index: u32) -> Result<char, &'static str> {
    match ty {
        LabelType::Alpha => {
            if index > 25 {
                Err("label index out of range for alphabetic resolution")
            } else {
                // unwrapping below safe due to previous check
                Ok(char::from_u32('a' as u32 + index).unwrap())
            }
        }
        LabelType::Num => {
            if index > 9 {
                Err("label index out of range for numeric resolution")
            } else {
                // unwrapping below safe due to previous check
                Ok(char::from_digit(index, 10).unwrap())
            }
        }
        LabelType::Rom => {
            todo!()
        }
    }
}

