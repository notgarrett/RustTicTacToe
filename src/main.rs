mod dec_tree;
mod iterative_deepening;
mod mcts;
mod minimax;
mod tictactoe;
mod ui;
use std::io;

use crate::{
    iterative_deepening::iterative_deepening,
    minimax::minimax,
    tictactoe::{GameStates, TicTacToe},
};

fn read_usize() -> usize {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}

fn main() -> io::Result<()> {
    println!("1 for Minimax, 2 for Iterative Deepening");

    let input = read_usize();
    let mut board = TicTacToe::new();

    if input == 1 {
        while board.state() == GameStates::Ongoing {
            board.print_board();
            loop {
                println!("What move would you like to make? (0 -> 8)");
                let game_move = read_usize();
                if board.can_play(game_move) {
                    board.play(game_move).unwrap();
                    if board.state() != GameStates::Ongoing {
                        break;
                    }
                    board.play(iterative_deepening(&board)).unwrap();
                }
            }
        }
        board.print_board();
        match board.state() {
            GameStates::Player1Win => println!("You win!"),
            GameStates::Player2Win => println!("You lose"),
            _ => println!("You draw."),
        };
    }

    if input == 2 {
        while board.state() == GameStates::Ongoing {
            board.print_board();
            loop {
                println!("What move would you like to make? (0 -> 8)");
                let game_move = read_usize();
                if board.can_play(game_move) {
                    board.play(game_move).unwrap();
                    if board.state() != GameStates::Ongoing {
                        break;
                    }
                    board.play(iterative_deepening(&board)).unwrap();
                    break;
                }
            }
        }
        board.print_board();
        match board.state() {
            GameStates::Player1Win => println!("You win!"),
            GameStates::Player2Win => println!("You lose"),
            _ => println!("You draw."),
        };
    }

    Ok(())
}
