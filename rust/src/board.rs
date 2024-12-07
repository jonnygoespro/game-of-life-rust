use std::cmp;
use std::cmp::PartialEq;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy, PartialEq)]
pub enum CellState {
    Alive,
    Dead,
    Zombie(i32)
}

#[wasm_bindgen]
pub struct Board {
    size: usize,
    cells: Vec<Vec<CellState>>,
}

impl Board {
    pub fn new(size: usize) -> Self {
        let mut cells = vec![vec![CellState::Dead; size]; size];

        for i in 0..size {
            for j in 0..size {
                cells[i][j] = match (i + j) % 2 {
                    0 => CellState::Alive,
                    _ => CellState::Dead
                }
            }
        }

        Board { cells, size }
    }

    pub fn get_state(&self) -> Vec<Vec<u8>> {
        self.cells
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&cell| match cell {
                        CellState::Alive => 0,
                        CellState::Dead => 1,
                        CellState::Zombie(_) => 2,
                    })
                    .collect()
            })
            .collect()
    }

    pub fn play_round(&mut self) {
        let old_cells = self.cells.clone();

        let mut new_cells = vec![vec![CellState::Dead; self.size]; self.size];

        for y in 0..self.size {
            for x in 0..self.size {
                let alive_neighbours = Board::get_alive_neighbours(y as isize, x as isize, &old_cells);

                match old_cells[y][x] {
                    CellState::Alive => {
                        match alive_neighbours {
                            2 | 3 => {
                                new_cells[y][x] = CellState::Alive
                            },
                            _ => {
                                new_cells[y][x] = CellState::Zombie(3)
                            }
                        }
                    },
                    CellState::Dead => {
                        match alive_neighbours {
                            3 => new_cells[y][x] = CellState::Alive,
                            _ => new_cells[y][x] = new_cells[y][x]
                        }
                    },
                    CellState::Zombie(0) => {
                        match alive_neighbours {
                            3 => new_cells[y][x] = CellState::Alive,
                            _ => new_cells[y][x] = CellState::Dead
                        }
                    },
                    CellState::Zombie(i) => {
                        match alive_neighbours {
                            3 => new_cells[y][x] = CellState::Alive,
                            _ => new_cells[y][x] = CellState::Zombie(cmp::max(0, i - 1))
                        }
                    }
                }
            }
        }

        self.cells = new_cells;
    }

    fn get_alive_neighbours(y: isize, x: isize, cells: &Vec<Vec<CellState>>) -> i32 {
        let mut alive_neighbours = 0;

        for dx in -1..=1 {
            for dy in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let pos_x = x + dx;
                let pos_y = y + dy;

                if pos_x >= 0 && pos_x < cells[0].len() as isize && pos_y >= 0 && pos_y < cells.len() as isize {
                    if cells[pos_y as usize][pos_x as usize] == CellState::Alive {
                        alive_neighbours += 1;
                    }
                }
            }
        }

        alive_neighbours
    }
}
