use std::io::Stdout;
use crate::board::{Board};

pub struct Game<'lifetime> {
    pub board: &'lifetime mut Board,
    pub alive: u64,
    pub ticks: u128,
}

struct Neighbours (bool, bool, bool, bool, bool, bool, bool, bool);

impl Game<'_> {
    pub fn new(board: &mut Board, alive: u64) -> Game {
        Game {
            board,
            alive,
            ticks: 0,
        }
    }

    pub fn get_cell_next_state(&self, size: usize, x: usize, y: usize) -> bool {
        let is_alive = self.board.grid[x][y];
        let mut num_neighbours = 0;

        let neighbours = Neighbours(
            x > 0 && y > 0 && self.board.grid[x - 1][y - 1],
            x > 0 && self.board.grid[x - 1][y],
            x > 0 && y < size - 1 && self.board.grid[x - 1][y + 1],
            y > 0 && self.board.grid[x][y - 1],
            y < size - 1 && self.board.grid[x][y + 1],
            x < size - 1 && y > 0 && self.board.grid[x + 1][y - 1],
            x < size - 1 && self.board.grid[x + 1][y],
            x < size - 1 && y < size - 1 && self.board.grid[x + 1][y + 1],
        );

        if neighbours.0 { num_neighbours += 1; }
        if neighbours.1 { num_neighbours += 1; }
        if neighbours.2 { num_neighbours += 1; }
        if neighbours.3 { num_neighbours += 1; }
        if neighbours.4 { num_neighbours += 1; }
        if neighbours.5 { num_neighbours += 1; }
        if neighbours.6 { num_neighbours += 1; }
        if neighbours.7 { num_neighbours += 1; }

        if is_alive {
            if num_neighbours < 2 || num_neighbours > 3 {
                return false
            }
            return true
        }

        if num_neighbours == 3 {
            return true
        }
        false
    }

    pub fn next_tick(&mut self) {
        // self.ticks += 1;
        for x in 0..self.board.size as usize {
            for y in 0..self.board.size as usize {
                let next_state = self.get_cell_next_state(self.board.size as usize, x, y);
                if next_state {
                    self.alive += 1;
                } else if self.alive > 0 {
                    self.alive -= 1;
                }
                self.board.set(x as u32, y as u32, next_state);
            }
        }
    }

    pub fn start(&mut self, stdout: &Stdout) {
        loop {
            self.next_tick();
            self.board.print(&stdout, self.board.size);
            std::thread::sleep(std::time::Duration::from_millis(50));
        }
    }
}