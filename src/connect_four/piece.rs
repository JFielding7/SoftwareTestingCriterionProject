use crate::connect_four::piece::Piece::{EMPTY, FIRST, SECOND};

#[derive(Clone)]
#[derive(Copy)]
#[derive(Eq, PartialEq, Hash)]
pub enum Piece {
    EMPTY,
    FIRST,
    SECOND,
}

const HASH_MOD: u64 = 1000000007;

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

    pub fn hash(&self, mut cell: usize) -> u64 {
        let val = match self {
            EMPTY => return 0,
            FIRST => 1,
            SECOND => 2,
        };

        let mut hash_pow = 1;
        let mut curr_pow = cell as u64;

        while cell != 0 {

            // println!("{cell}");

            if (cell & 1) == 1 {
                hash_pow = (hash_pow * curr_pow) % HASH_MOD;
            }

            curr_pow = (curr_pow * curr_pow) % HASH_MOD;
            cell >>= 1;
        }

        (val * hash_pow) % HASH_MOD
    }
}
