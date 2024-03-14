use crate::tictactoe::{GameStates, TicTacToe};

fn minimax(position: &TicTacToe) -> usize {
    let (mut to_play, mut max) = (0 as usize, -8);

    for i in 0..9 {
        if position.can_play(i) && position.check_winning_move(i) {
            return i;
        }
    }

    for i in 0..9 {
        if position.can_play(i) {
            let mut clone = *position;
            clone.play(i).unwrap();
            let score = -minimax_helper(&clone, -10000000, 10000000);
            if score > max {
                max = score;
                to_play = i;
            }
        }
    }

    to_play
}

fn minimax_helper(position: &TicTacToe, mut alpha: i32, mut beta: i32) -> i32 {
    // Check winning position

    if position.state() != GameStates::Ongoing {
        return 0;
    }

    for i in 0..9 {
        if position.can_play(i) && position.check_winning_move(i) {
            return 10 - position.moves() as i32;
        }
    }

    let max = (8 - position.moves() as i32) / 2;
    if beta > max {
        beta = max;
        if alpha >= beta {
            return beta;
        }
    }

    // Begin simulating positions

    for i in 0..9 {
        if position.can_play(i) {
            let mut clone = *position;
            clone.play(i).unwrap();
            let score = -minimax_helper(&clone, -beta, -alpha);
            if score >= beta {
                return beta;
            } else if score > alpha {
                alpha = score;
            }
        }
    }
    alpha
}

// 0 1 2
// 3 4 5
// 6 7 8

#[cfg(test)]
mod tests {
    use crate::tictactoe::TicTacToe;

    use super::minimax;

    #[test]
    fn test_minimax_easy() {
        let game = TicTacToe::from("3142");
        let to_play = minimax(&game);
        assert_eq!(to_play, 5);
    }

    #[test]
    fn test_minimax_medium() {
        let game = TicTacToe::from("012");
        let to_play = minimax(&game);
        assert_eq!(to_play, 4);
    }
}
