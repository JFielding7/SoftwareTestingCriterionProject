use crate::connect_four::piece::Piece;
use crate::connect_four::piece::Piece::{EMPTY, FIRST, SECOND};
use std::fmt;
use std::hash::{Hash, Hasher};
use crate::connect_four::solver_util;
use crate::connect_four::solver_util::{BOARD_SIZE, COLS, DEFAULT_MOVE_ORDER, EMPTY_CELL, FIRST_PLAYER, ROWS, SECOND_PLAYER};
use crate::connect_four::state::State;


#[derive(Clone)]
pub struct StateArray {
    board: [Piece; BOARD_SIZE],
    current_player: Piece,
    last_move: usize,
    moves_made: usize,
    curr_hash: u64, // iterative build hash, much faster than default Hash
}

impl Eq for StateArray {}

impl PartialEq for StateArray {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Hash for StateArray {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.curr_hash.hash(state);
    }
}

impl State for StateArray {
    fn start_state() -> Self {
        Self {
            board: [EMPTY; BOARD_SIZE],
            current_player: FIRST,
            last_move: 0,
            moves_made: 0,
            curr_hash: 0,
        }
    }

    fn is_win(&self) -> bool {
        let c = self.last_move / ROWS;
        let r = self.last_move % ROWS;

        const DELTA: [(i32, i32); 4] = [(1, 0), (0, 1), (1, -1), (1, 1)];
        let piece = self.current_player.next_player();

        for (mut dr, mut dc) in DELTA {
            let mut piece_count = 0;

            for _ in 0..2 {
                let mut r1 = r as i32;
                let mut c1 = c as i32;

                loop {
                    r1 += dr;
                    c1 += dc;

                    if r1 < 0 || r1 >= ROWS as i32
                        || c1 < 0 || c1 >= COLS as i32
                        || self.piece_at(r1, c1) != piece
                    {
                        break
                    }

                    piece_count += 1;
                }

                dr = -dr;
                dc = -dc;
            }

            if piece_count >= 3 {
                return true
            }
        }

        false
    }

    fn board_full(&self) -> bool {
        self.moves_made == BOARD_SIZE
    }

    fn moves_made(&self) -> usize {
        self.moves_made
    }

    fn max_eval(&self) -> i32 {
        (((BOARD_SIZE + 1) - self.moves_made) >> 1) as i32
    }

    fn play_move(&self, col: usize) -> Option<Self> {
        let mut next_move = col * ROWS;

        while self.board[next_move].is_occupied() {
            next_move += 1;

            if next_move % ROWS == 0 {
                return None
            }
        }

        let mut next_board = self.board;
        next_board[next_move] = self.current_player;

        Some(Self {
            board: next_board,
            current_player: self.current_player.next_player(),
            last_move: next_move,
            moves_made: self.moves_made + 1,
            curr_hash: self.current_player.hash(self.curr_hash, next_move)
        })
    }

    fn next_states(&self) -> Vec<Self> {
        let mut next_states = vec![];

        for col in DEFAULT_MOVE_ORDER {
            let next_move = self.play_move(col);

            match next_move {
                None => continue,
                Some(state) => next_states.push(state)
            }
        }

        next_states
    }

    fn encode(board: &Vec<String>) -> Self {

        let mut game_state = Self::start_state();

        for c in 0..COLS {

            for r in 0..ROWS {
                let row = board[ROWS - 1 - r].as_bytes();

                if c >= row.len() {
                    break
                }

                let piece = row[c] as char;
                let cell = c * ROWS + r;

                match piece {
                    FIRST_PLAYER => {
                        game_state.board[cell] = FIRST;
                        game_state.moves_made += 1;
                    },
                    SECOND_PLAYER => {
                        game_state.board[cell] = SECOND;
                        game_state.moves_made += 1;
                    },
                    _ => {}
                };
            }
        }

        if (game_state.moves_made & 1) == 0 {
            game_state.current_player = FIRST;
        } else {
            game_state.current_player = SECOND;
        }
        
        game_state
    }

    fn decode(&self) -> String {
        let mut board_str = String::new();

        for r in (0..ROWS).rev() {

            for c in 0..COLS {

                let cell = c * ROWS + r;

                board_str.push(match self.board[cell] {
                    EMPTY => EMPTY_CELL,
                    FIRST => FIRST_PLAYER,
                    SECOND => SECOND_PLAYER,
                });
            }

            board_str.push('\n');
        }

        board_str
    }
}

impl StateArray {

    fn board_index(row: usize, col: usize) -> usize {
        col * ROWS + row
    }

    fn piece_at(&self, row: i32, col: i32) -> Piece {
        self.board[Self::board_index(row as usize, col as usize)]
    }
}

impl fmt::Display for StateArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decode())
    }
}
