use crate::gviz::GvizNode;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum PlayerColor {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct PlayerMove {
    turn: usize,
    player: PlayerColor,
    mov: String,
}

impl PlayerMove {
    pub fn new(turn: usize, player: PlayerColor, mov: &str) -> Self {
        Self {
            turn: turn,
            player: player,
            mov: mov.trim().to_string(),
        }
    }
}

impl GvizNode for PlayerMove {
    fn id(&self) -> String {
        format!("{:?}_{:?}_{}",
            self.player,
            self.turn,
            self.mov,
        )
    }

    fn id_with_attributes(&self) -> String {
        format!("{} [shape=circle; style=filled; fillcolor={}; label={}; fontcolor={}];",
            self.id(),
            match self.player {
                PlayerColor::White => "white",
                PlayerColor::Black => "black",
            },
            self.mov,
            match self.player {
                PlayerColor::White => "black",
                PlayerColor::Black => "white",
            },
        )
    }

    
}