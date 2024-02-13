use std::io::Stdout;
use crate::board::{Board};

pub struct Game<'lifetime> {
    pub board: &'lifetime mut Board,
    pub alive: u64,
    pub ticks: u128,
}


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
        self.ticks += 1;
        for x in 0..self.board.size as usize {
            for y in 0..self.board.size as usize {
                let next_state = self.get_cell_next_state(self.board.size as usize, x, y);
                if next_state {
                    self.alive += 1;
                } else {
                    self.alive -= 1;
                }
                self.board.set(x as u32, y as u32, next_state);
            }
        }
    }

    pub fn start(&mut self, mut stdout: &Stdout) {
        loop {
            self.next_tick();
            self.board.print(&stdout, self.board.size);
            std::thread::sleep(std::time::Duration::from_millis(100));
        }
    }
}