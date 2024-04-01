// is this thing going to have to be a class?

use crate::tictactoe::{GameError, GameStates, TicTacToe};

struct MCTS_Node {
    state: TicTacToe,
    children: std::vec::Vec<MCTS_Node>,
    visits: i32,
    wins: i32,
    parent: Option<Box<MCTS_Node>>,
}

impl MCTS_Node {
    pub fn new(state: TicTacToe) -> Self {
        Self {
            state,
            children: std::vec::Vec::new(),
            visits: 0,
            wins: 0,
            parent: None,
        }
    }

    pub fn play(&mut self, pos: usize) -> Result<(), GameError> {
        let mut clone = self.state.clone();

        if !self.state.can_play(pos) {
            return Err(GameError::InvalidMove);
        }

        if let GameStates::Player1Win | GameStates::Player2Win = clone.play(pos)? {
            self.propegate_wins();
        };
        self.visits += 1;
        self.add_child(MCTS_Node::new(clone));
        Ok(())
    }

    pub fn propegate_wins(&mut self) {
        self.wins += 1;
        if let None = &self.parent {
            ()
        } else {
            self.parent.as_mut().unwrap().propegate_wins()
        }
    }

    pub fn add_child(&mut self, child: MCTS_Node) {
        self.children.push(child);
    }

    fn is_terminal(self) -> bool {
        self.state.state() != GameStates::Ongoing
    }

    fn bcu_vector(&self) -> std::vec::Vec<(&MCTS_Node, f32)> {
        let mut vec = std::vec::Vec::new();
        for node in &self.children {
            vec.push((node, MCTS_Node::bcu(node)));
            // bc1 on Child
        }
        vec
    }

    fn bcu(node: &MCTS_Node) -> f32 {
        let c = 10.0;
        let wi = node.wins as f32;
        let ni = node.visits as f32;
        let parent_ni_boxed = &node.parent;
        let parent_ni = parent_ni_boxed.as_ref().unwrap().visits.ilog2() as f32;
        let bcu1 = (wi / ni) + c * (parent_ni / ni).sqrt();
        bcu1
    }
}
