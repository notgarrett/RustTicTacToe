use std::io::{stdout, Stdout};

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};

use crate::tictactoe::{self, GameStates, TicTacToe};
use std::io::Result;

enum UIState {
    Ongoing,
    Over,
}

pub struct TicTacToeUI {
    game: TicTacToe,
    state: UIState,
    display: Terminal<CrosstermBackend<Stdout>>,
    spots: [bool; 9],
}

enum UIError {
    LoadFailure,
}

impl TicTacToeUI {
    pub fn new() -> Self {
        let display = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        Self {
            game: TicTacToe::new(),
            state: UIState::Ongoing,
            display,
            spots: [false; 9],
        }
    }

    pub fn init(&mut self) -> Result<()> {
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        self.display.clear()?;

        loop {
            self.display.draw(|frame| {
                let area = frame.size();
                frame.render_widget(
                    Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                        .white()
                        .on_blue(),
                    area,
                );
            })?;

            if event::poll(std::time::Duration::from_millis(16))? {
                if let event::Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    pub fn play(&mut self, pos: usize) -> GameStates {
        self.game.play(pos).unwrap()
    }
}
