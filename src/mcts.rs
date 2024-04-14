// Wasnt able to get MCTS working.

use crate::tictactoe::{CurrentTurn, GameError, GameStates, TicTacToe};
use rand::Rng;
use std::{borrow::BorrowMut, rc::Rc};

#[derive(PartialEq)]
struct MctsNode {
    state: TicTacToe,
    children: std::vec::Vec<Box<MctsNode>>,
    visits: i32,
    wins: i32,
    turn: CurrentTurn,
    parent: Option<Box<MctsNode>>,
}

impl MctsNode {
    pub fn new(state: TicTacToe, turn: CurrentTurn) -> Self {
        Self {
            state,
            children: std::vec::Vec::new(),
            visits: 0,
            wins: 0,
            turn,
            parent: None,
        }
    }

    pub fn set_parent(&mut self, parent: Box<MctsNode>) {
        self.parent = Some(parent);
    }

    pub fn generate_moves(&mut self) {
        // First we generate all possible moves at depth 1.

        let array_of_moves = [0, 1, 2, 3, 4, 5, 6, 7, 8];

        for i in array_of_moves {
            self.play(i);
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
        let mut child = MctsNode::new(clone, self.turn);
        // child.set_parent(self);

        self.add_child(MctsNode::new(clone, self.turn));

        Ok(())
    }

    pub fn generate_tree(&mut self) {
        // Check if we have to do a rollout.
    }

    pub fn rollout(&mut self) -> GameStates {
        let mut rng = rand::thread_rng();
        let mut random_number = rng.gen_range(0..8) as usize;
        let mut game_clone = self.state.clone();
        while game_clone.state() != GameStates::Ongoing {
            if game_clone.can_play(random_number) {
                game_clone.play(random_number).unwrap();
            };
            random_number = rng.gen_range(0..8) as usize;
        }

        game_clone.state()
    }

    pub fn propegate_wins(&mut self) {
        self.wins += 1;
        if let None = &self.parent {
            ()
        } else {
            self.parent.as_mut().unwrap().propegate_wins();
        }
    }

    pub fn add_child(&mut self, child: MctsNode) {
        self.children.push(Box::new(child));
    }

    fn is_terminal(self) -> bool {
        self.state.state() != GameStates::Ongoing
    }

    fn bcu_vector(&self) -> std::vec::Vec<f32> {
        let mut vec = std::vec::Vec::new();
        for node in &self.children {
            vec.push(MctsNode::bcu(&node));
        }
        vec
    }

    fn bcu(node: &MctsNode) -> f32 {
        let c = 10.0;
        let wi = node.wins as f32;
        let ni = node.visits as f32;
        let parent_ni_boxed = &node.parent;
        let parent_ni = parent_ni_boxed.as_ref().unwrap().visits.ilog2() as f32;
        let bcu1 = (wi / ni) + c * (parent_ni / ni).sqrt();
        bcu1
    }
}
