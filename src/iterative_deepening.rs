use super::tictactoe::{GameStates, TicTacToe};

pub fn iterative_deepening(board: &TicTacToe) -> usize {
    let mut depth: usize = 0;

    // Check immediate wins
    for i in 0..9 {
        if board.can_play(i) && board.check_winning_move(i) {
            return i;
        }
    }

    let mut to_play = 0;
    loop {
        // Loop a max 9 times to calculate all depths.
        let mut best = -8;
        depth += 1;
        // Simulate moves.
        for i in 0..9 {
            if board.can_play(i) {
                let mut clone = *board;
                clone.play(i).unwrap();
                let score = -depth_limited_search(&clone, depth);
                if score > best {
                    best = score;
                    to_play = i;
                }
            }
        }
        if depth >= 9 {
            break;
        }
        // This is a winning score, if calculated, we break the search and return this move
        // immediately.
        if best >= (10 - board.moves() - depth) as i32 {
            break;
        }
    }

    to_play
}

fn depth_limited_search(board: &TicTacToe, depth: usize) -> i32 {
    if board.state() != GameStates::Ongoing || depth == 0 {
        return 0;
    }

    // Check immediate wins
    for i in 0..9 {
        if board.can_play(i) && board.check_winning_move(i) {
            return 10 - board.moves() as i32;
        }
    }

    let mut best = -8;

    for i in 0..9 {
        if board.can_play(i) {
            let mut clone = *board;
            clone.play(i).unwrap();
            let score = -depth_limited_search(&clone, depth - 1);
            if score > best {
                best = score;
            }
        }
    }

    best
}
