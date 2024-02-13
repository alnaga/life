use std::io::stdout;

mod board;
mod game;
mod utils;

// Any live cell with fewer than two live neighbors dies, as if by underpopulation.
// Any live cell with two or three live neighbors lives on to the next generation.
// Any live cell with more than three live neighbors dies, as if by overpopulation.
// Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.

fn main() {
    let mut stdout = stdout();
    let size = utils::parse_u16_input("Enter the size of the board: ");
    let probability = utils::parse_f64_input("What probability for aliveness do you want to use: ");
    let mut game_board = board::Board::new(size, probability);
    let mut game = game::Game::new(&mut game_board);
    game.start(&mut stdout);
}