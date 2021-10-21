
#[derive(Debug, Clone)]
pub struct Cell {
    pub row: u32,
    pub col: u32,
    pub north: Option<(u32,u32)>,
    pub south: Option<(u32,u32)>,
    pub east: Option<(u32,u32)>,
    pub west: Option<(u32,u32)>,
    pub links: Vec<(u32, u32)>,
}

impl Cell {
    pub fn new(row: u32, col: u32, limits:(u32, u32)) -> Self {
        let north = if row == 0 {
            None
        } else {
            Some((row - 1, col))
        };
        let west = if col == 0 {
            None
        } else {
            Some((row, col - 1))
        };
        let south = if row >= limits.0 -1 {
            None
        } else {
            Some((row + 1, col))
        };
        let east = if col >= limits.1 -1 {
            None
        } else {
            Some((row, col + 1))
        };

        Cell {
            row,
            col,
            north,
            south,
            west,
            east,
            links: vec![],
        }
    }

    pub fn link(&mut self, to:(u32, u32)) {
        self.links.push(to);
    }

    pub fn is_linked(&self, to:(u32, u32)) -> bool {
        self.links.contains(&to)
    }
}
