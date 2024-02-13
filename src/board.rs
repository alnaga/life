use std::{io::{Stdout, Write}};
use rand::{Rng, thread_rng};
use crossterm::{cursor, ExecutableCommand, terminal};

pub struct Board {
    pub grid: Vec<Vec<bool>>,
    pub size: u16,
}

impl Board {
    pub fn new (size: u16, probability: f64) -> Board {
        let mut rng = thread_rng();
        let mut board = Vec::new();
        for _ in 0..size {
            let mut row = Vec::new();
            for _ in 0..size {
                let value = rng.gen_bool(probability);
                row.push(value);
            }
            board.push(row);
        }
        Board {
            grid: board,
            size,
        }
    }

    pub fn set (&mut self, x: u32, y: u32, value: bool) {
        self.grid[x as usize][y as usize] = value;
    }

    pub fn print (&self, mut stdout: &Stdout, num_rows: u16, ticks: u128) {
        let mut new_display = String::new();

        for row in &self.grid {
            let mut line = String::from("");
            for cell in row {
                if *cell {
                    line.push(std::char::from_u32(0x2588).unwrap_or_else(|| '�'));
                } else {
                    line.push(std::char::from_u32(0x2581).unwrap_or_else(|| '�'));
                }
            }
            new_display.push_str(&line);
            new_display.push_str("\n");
        }
        // Overwrite the previously displayed board + the newline
        stdout.execute(cursor::MoveUp(num_rows + 2)).unwrap();
        writeln!(stdout, "{}", new_display).unwrap();
        writeln!(stdout, "Ticks: {}", ticks).unwrap();
    }
}