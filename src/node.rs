use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node {
    x: i32,
    y: i32,
    block: i32,
    parent: Option<Box<Node>>,
    g: i32,
    h: i32,
    f: i32,
    allowed_directions: Vec<(i32, i32)>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            x: 0,
            y: 0,
            block: 0,
            parent: None,
            f: 0,
            g: 0,
            h: 0,
            allowed_directions: vec![(-1, 0), (1, 0), (0, -1), (0, 1)],
        }
    }
}

impl Node {
    pub fn new(
        x: i32,
        y: i32,
        block: i32,
        parent: Option<Box<Node>>,
        allowed_directions: Option<Vec<(i32, i32)>>,
    ) -> Self {
        Node {
            x,
            y,
            block,
            parent,
            g: 0,
            h: 0,
            f: 0,
            allowed_directions: allowed_directions
                .unwrap_or_else(|| vec![(-1, 0), (1, 0), (0, -1), (0, 1)]),
        }
    }

    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn get_block(&self) -> i32 {
        self.block
    }
    pub fn get_f(&self) -> i32 {
        self.f
    }
    pub fn get_parent(&self) -> Option<&Node> {
        self.parent.as_deref()
    }
    pub fn get_g(&self) -> i32 {
        self.g
    }
    pub fn get_h(&self) -> i32 {
        self.h
    }
    pub fn get_allowed_directions(&self) -> &[(i32, i32)] {
        &self.allowed_directions
    }

    pub fn set_g(&mut self, g: i32) {
        self.g = g
    }
    pub fn set_h(&mut self, h: i32) {
        self.h = h
    }
    pub fn set_f(&mut self, f: i32) {
        self.f = f
    }
    pub fn set_x(&mut self, x: i32) {
        self.x = x
    }
    pub fn set_y(&mut self, y: i32) {
        self.y = y
    }
    pub fn set_parent(&mut self, parent: Option<Box<Node>>) {
        self.parent = parent
    }
    pub fn set_block(&mut self, block: i32) {
        self.block = block
    }
    pub fn set_allowed_directions(&mut self, allowed_directions: Vec<(i32, i32)>) {
        self.allowed_directions = allowed_directions
    }
}
