mod minimax;
mod tictactoe;
mod ui;
use std::io;
use ui::TicTacToeUI;

fn main() -> io::Result<()> {
    println!("Hello, world!");
    let mut test = TicTacToeUI::new();
    test.init().unwrap();
    Ok(())
}
