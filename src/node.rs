#![allow(dead_code)]
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
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
            allowed_directions: allowed_directions.unwrap_or_else(|| {
                vec![
                    (-1, 0), // West
                    (1, 0),  // East
                    (0, -1), // North
                    (0, 1),  // South
                ]
            }),
        }
    }

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn block(&self) -> i32 {
        self.block
    }

    pub fn f(&self) -> i32 {
        self.f
    }

    pub fn parent(&self) -> Option<&Node> {
        self.parent.as_deref()
    }

    pub fn g(&self) -> i32 {
        self.g
    }

    pub fn h(&self) -> i32 {
        self.h
    }

    pub fn allowed_directions(&self) -> &[(i32, i32)] {
        &self.allowed_directions
    }

    pub fn set_g(&mut self, g: i32) {
        self.g = g;
    }

    pub fn set_h(&mut self, h: i32) {
        self.h = h;
    }

    pub fn set_f(&mut self, f: i32) {
        self.f = f;
    }

    pub fn set_x(&mut self, x: i32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: i32) {
        self.y = y;
    }

    pub fn set_parent(&mut self, parent: Option<Box<Node>>) {
        self.parent = parent;
    }

    pub fn set_block(&mut self, block: i32) {
        self.block = block;
    }

    pub fn set_allowed_directions(&mut self, allowed_directions: Vec<(i32, i32)>) {
        self.allowed_directions = allowed_directions;
    }
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.f.cmp(&other.f)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
