#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Node {
    x: usize,
    y: usize,
    block: u8,
    allowed_directions: Vec<(isize, isize)>,
    parent: Option<Box<Node>>,
    g: usize,
    h: usize,
    f: usize,
}

impl Node {
    pub fn new(x: usize, y: usize, block: u8, allowed_directions: Vec<(isize, isize)>) -> Self {
        Node {
            x,
            y,
            block,
            allowed_directions,
            parent: None,
            g: 0,
            h: 0,
            f: 0,
        }
    }

    pub fn get_x(&self) -> usize {
        self.x
    }
    pub fn get_y(&self) -> usize {
        self.y
    }
    pub fn get_block(&self) -> u8 {
        self.block
    }
    pub fn get_allowed_directions(&self) -> &Vec<(isize, isize)> {
        &self.allowed_directions
    }
    pub fn get_parent(&self) -> Option<&Box<Node>> {
        self.parent.as_ref()
    }
    pub fn set_parent(&mut self, parent: Node) {
        self.parent = Some(Box::new(parent));
    }
    pub fn get_f(&self) -> usize {
        self.f
    }
    pub fn set_f(&mut self, f: usize) {
        self.f = f;
    }
    pub fn get_g(&self) -> usize {
        self.g
    }
    pub fn set_g(&mut self, g: usize) {
        self.g = g;
    }
    pub fn get_h(&self) -> usize {
        self.h
    }
    pub fn set_h(&mut self, h: usize) {
        self.h = h;
    }
}
