#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn black() -> Self {
        Self::new(0, 0, 0)
    }

    pub fn white() -> Self {
        Self::new(255, 255, 255)
    }
}


pub struct Renderable {
    pub color: Color,
    pub content: String,
}

#[derive(Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn origin() -> Self {
        Self::new(0, 0)
    }
}

pub struct Character {
    pub states: Vec<Renderable>,
    pub state: usize,
    pub position: Position,
}


pub struct Game {
    pub characters: Vec<Character>,
    pub stopped: bool,

    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            characters: vec![],
            stopped: false,
            width, height }
    }

    pub fn set_size(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
    }

    pub fn process_key(&mut self, key: char, ctrl: bool) {
        if ctrl && key == 'q' {
            self.stopped = true;
            return;
        }
    }
}