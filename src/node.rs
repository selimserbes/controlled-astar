#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Node {
    x: usize,
    y: usize,
    block: u8,
    allowed_directions: Vec<(isize, isize)>,
    parent: Option<Box<Node>>, // Önceki versiyon
    g: usize,
    h: usize,
    f: usize,
}

impl Node {
    pub fn new(
        x: usize,
        y: usize,
        block: Option<u8>,
        allowed_directions: Option<Vec<(isize, isize)>>,
        parent: Option<Node>, // Güncellenmiş versiyon
    ) -> Self {
        Node {
            x,
            y,
            block: block.unwrap_or(0),
            allowed_directions: allowed_directions
                .unwrap_or_else(|| vec![(1, 0), (-1, 0), (0, 1), (0, -1)]),
            parent: parent.map(Box::new), // `Option<Node>`'i `Option<Box<Node>>`'e dönüştür
            g: 0,
            h: 0,
            f: 0,
        }
    }

    // Getter ve Setter'lar
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
    pub fn get_parent(&self) -> Option<&Node> {
        self.parent.as_deref() // `Option<Box<Node>>` yerine `Option<&Node>` döner
    }
    pub fn set_parent(&mut self, parent: Box<Node>) {
        self.parent = Some(parent);
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
