use crate::piece::Piece::{EMPTY, FIRST, SECOND};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq, PartialEq)]
pub enum Piece {
    EMPTY,
    FIRST,
    SECOND,
}

impl Piece {
    pub fn next_player(&self) -> Self {
        match self {
            EMPTY => EMPTY,
            FIRST => SECOND,
            SECOND => FIRST,
        }
    }
    
    pub fn is_occupied(&self) -> bool {
        self != &EMPTY
    }
}
