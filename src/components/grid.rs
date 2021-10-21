use log::info;
use rand::prelude::*;
use std::fmt;
use std::vec;

use crate::components::cell::Cell;

#[derive(Debug, Clone)]
pub struct Grid {
    pub cols: u32,
    pub rows: u32,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut cells = vec![];

        for i in 0..rows {
            for j in 0..cols {
                let cell = Cell::new(i, j, (cols, rows));
                cells.push(cell);
            }
        }

        Grid { cols, rows, cells }
    }
    fn link_cells(&mut self, from: (u32, u32), to: (u32, u32), bidi: bool) {
        let from_ind = self.get_cell_index(from) as usize;

        self.cells[from_ind].link(to);

        if bidi {
            let to_ind = self.get_cell_index(to) as usize;

            self.cells[to_ind].link(from);
        }
    }

    fn get_cell_index(&self, pos: (u32, u32)) -> u32 {
        pos.0 * self.rows + pos.1
    }
    pub fn binary_tree(&mut self) {
        for i in 0..self.cells.len() {
            let cell = &self.cells[i];
            match (cell.south, cell.east) {
                (Some(pos), None) | (None, Some(pos)) => {
                    self.link_cells((cell.row, cell.col), (pos.0, pos.1), true)
                }
                (Some(n_pos), Some(e_pos)) => {
                    let mut rng = rand::thread_rng();
                    let rn: f64 = rng.gen();
                    if rn < 0.5 {
                        self.link_cells((cell.row, cell.col), (n_pos.0, n_pos.1), true);
                    } else {
                        self.link_cells((cell.row, cell.col), (e_pos.0, e_pos.1), true);
                    }
                }
                (None, None) => (),
            }
        }
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output: String = String::from("+");
        for _ in 0..self.cols {
            output.push_str("---+");
        }

        output.push('\n');

        for row in 0..self.rows {
            let mut body = String::from("");
            let mut body_floor = String::from("");
            let corner = "+";

            for col in 0..self.cols {
                let cell = &self.cells[self.get_cell_index((row, col)) as usize];

                body.push_str("   ");
                if let Some(pos) = cell.east {
                    if cell.is_linked(pos) {
                        body.push(' ');
                    } else {
                        body.push('|');
                    }
                }

                if let Some(pos) = cell.south {
                    if cell.is_linked(pos) {
                        body_floor.push_str("   ");
                    } else {
                        body_floor.push_str("---");
                    }
                } else {
                    body_floor.push_str("---");
                }
                body_floor.push_str(corner);
            }

            output.push_str(&format!("|{}|\n", body));

            output.push_str(&format!("{}{}\n", corner, body_floor));
        }

        write!(f, "{}", output)
    }
}
