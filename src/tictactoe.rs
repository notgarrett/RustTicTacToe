#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GameStates {
    Ongoing,
    Player1Win,
    Player2Win,
    Draw,
}

#[derive(Clone, Copy, PartialEq)]
enum CellStates {
    Player1,
    Player2,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CurrentTurn {
    Player1,
    Player2,
}

#[derive(Debug, PartialEq)]
pub enum GameError {
    InvalidMove,
    OutOfBounds,
    GameOver,
}

#[derive(Clone, Copy, PartialEq)]
pub struct TicTacToe {
    board: [CellStates; 9],
    turn: CurrentTurn,
    state: GameStates,
    moves: usize,
}

impl From<&str> for TicTacToe {
    fn from(value: &str) -> Self {
        let mut game = Self::new();
        value.chars().for_each(|char| {
            game.play(char.to_digit(10).unwrap() as usize).unwrap();
        });
        game
    }
}

const WIDTH: usize = 3;
const HEIGHT: usize = 3;

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [CellStates::Empty; 9],
            turn: CurrentTurn::Player1,
            state: GameStates::Ongoing,
            moves: 0,
        }
    }

    pub fn play(&mut self, pos: usize) -> Result<GameStates, GameError> {
        if self.state != GameStates::Ongoing {
            return Err(GameError::GameOver);
        }

        if pos > 8 {
            return Err(GameError::OutOfBounds);
        }

        if let CellStates::Player1 | CellStates::Player2 = self.board[pos] {
            return Err(GameError::InvalidMove);
        }

        self.board[pos] = match self.turn {
            CurrentTurn::Player1 => CellStates::Player1,
            CurrentTurn::Player2 => CellStates::Player2,
        };

        self.moves += 1;

        let state = self.eval();

        self.turn = match self.turn {
            CurrentTurn::Player1 => CurrentTurn::Player2,
            _ => CurrentTurn::Player1,
        };

        Ok(state)
    }

    pub fn turn(&self) -> CurrentTurn {
        self.turn
    }

    pub fn check_winning_move(&self, pos: usize) -> bool {
        let mut clone = *self;

        if let GameStates::Player1Win | GameStates::Player2Win = clone.play(pos).unwrap() {
            true
        } else {
            false
        }
    }

    pub fn can_play(&self, pos: usize) -> bool {
        pos < 9 && self.board[pos] == CellStates::Empty
    }

    pub fn moves(&self) -> usize {
        self.moves
    }

    pub fn state(&self) -> GameStates {
        self.state
    }

    fn eval(&mut self) -> GameStates {
        if self.check_win() {
            match self.turn {
                CurrentTurn::Player1 => self.state = GameStates::Player1Win,
                _ => self.state = GameStates::Player2Win,
            }
        } else if self.moves == 9 {
            self.state = GameStates::Draw
        }
        self.state
    }

    fn check_win(&self) -> bool {
        self.check_vertical_wins() || self.check_diagonal_wins() || self.check_horizontal_wins()
    }

    fn check_horizontal_wins(&self) -> bool {
        for i in 0..WIDTH as usize {
            if self.board[0 + 3 * i] != CellStates::Empty
                && self.board[0 + 3 * i] == self.board[1 + 3 * i]
                && self.board[1 + 3 * i] == self.board[2 + 3 * i]
            {
                return true;
            }
        }
        false
    }

    fn check_vertical_wins(&self) -> bool {
        for i in 0..HEIGHT as usize {
            if self.board[0 + i] != CellStates::Empty
                && self.board[0 + i] == self.board[3 + i]
                && self.board[3 + i] == self.board[6 + i]
            {
                return true;
            }
        }
        false
    }

    fn check_diagonal_wins(&self) -> bool {
        (self.board[0] != CellStates::Empty
            && self.board[0] == self.board[4]
            && self.board[4] == self.board[8])
            || (self.board[2] != CellStates::Empty
                && self.board[2] == self.board[4]
                && self.board[4] == self.board[6])
    }

    pub fn print_board(&self) {
        let mut count = 1;
        for i in self.board {
            match i {
                CellStates::Empty => print!("|  |"),
                CellStates::Player1 => print!("| x |"),
                CellStates::Player2 => print!("| o |"),
            };
            if count % 3 == 0 {
                println!();
            }
            count += 1;
        }
    }
}

// 0 1 2
// 3 4 5
// 6 7 8

#[cfg(test)]
mod tests {
    use crate::tictactoe::{GameError, GameStates, TicTacToe};
    #[test]
    fn test_horizontal_win() {
        let mut game = TicTacToe::from("0817");
        assert_eq!(game.play(2).unwrap(), GameStates::Player1Win);

        let mut game_2 = TicTacToe::from("3847");
        assert_eq!(game_2.play(5).unwrap(), GameStates::Player1Win);

        let mut game_3 = TicTacToe::from("3847");
        assert_eq!(game_3.play(5).unwrap(), GameStates::Player1Win);
    }

    #[test]
    fn check_ungoing_state() {
        let mut game = TicTacToe::new();
        assert_eq!(game.play(4).unwrap(), GameStates::Ongoing);
        assert_eq!(game.play(8).unwrap(), GameStates::Ongoing);
        assert_eq!(game.play(5).unwrap(), GameStates::Ongoing);
        assert_eq!(game.play(6).unwrap(), GameStates::Ongoing);
    }

    #[test]
    fn test_vertical_win() {
        let mut game = TicTacToe::from("0837");
        assert_eq!(game.play(6).unwrap(), GameStates::Player1Win);

        let mut game_2 = TicTacToe::from("1245");
        assert_eq!(game_2.play(7).unwrap(), GameStates::Player1Win);

        let mut game_3 = TicTacToe::from("2153");
        assert_eq!(game_3.check_winning_move(8), true);
        assert_eq!(game_3.play(8).unwrap(), GameStates::Player1Win);
    }

    #[test]
    fn test_diagonal_win() {
        let mut game = TicTacToe::from("0145");
        assert_eq!(game.play(8).unwrap(), GameStates::Player1Win);

        let mut game_2 = TicTacToe::from("2547");
        assert_eq!(game_2.play(6).unwrap(), GameStates::Player1Win);
    }

    // 0 1 2
    // 3 4 5
    // 6 7 8

    #[test]
    fn test_overflow() {
        let mut game = TicTacToe::from("048536217");
        assert_eq!(game.play(2).expect_err(""), GameError::GameOver)
    }

    #[test]
    fn test_invalidinput() {
        let mut game = TicTacToe::new();
        game.play(4).unwrap();
        assert_eq!(game.play(4).expect_err(""), GameError::InvalidMove)
    }

    #[test]
    fn test_outofbounds() {
        let mut game = TicTacToe::new();
        assert_eq!(game.play(9).expect_err(""), GameError::OutOfBounds)
    }
}
