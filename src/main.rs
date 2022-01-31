extern crate ncurses;
mod board;

pub mod cell;
fn main() {
    let mut board = board::Board::new();
    board.game_loop()
}
