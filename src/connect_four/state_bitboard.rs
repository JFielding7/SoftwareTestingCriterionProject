use std::fmt;
use crate::connect_four::piece::Piece::EMPTY;
use crate::connect_four::solver_util::{BOARD_SIZE, COLS, EMPTY_CELL, FIRST_PLAYER, ROWS, SECOND_PLAYER};
use crate::connect_four::state::State;

pub const BOARD_BITS: usize = 49;
pub const BOARD_MASK: u64 = (1 << BOARD_BITS) - 1;
pub const COL_BITS: usize = ROWS + 1;
pub const COL_MASK: u64 = (1 << COL_BITS) - 1;
const CONNECTION_DIRECTIONS: &[i32; 4] = &[1, 6, 7, 8];
pub const IS_LEGAL: u64 = 0b0111111011111101111110111111011111101111110111111;
const DEFAULT_MOVE_ORDER: [usize; COLS] = [3, 2, 4, 1, 5, 0, 6];


#[derive(Eq, PartialEq, Clone, Hash)]
pub struct StateBitboard {
    pub curr_pieces: u64,
    pub opp_pieces: u64,
    pub height_map: u64,
    pub moves_made: usize
}

impl State for StateBitboard {

    fn start_state() -> Self {
        Self::encode(&vec![" ".repeat(COLS); ROWS])
    }

    fn is_win(&self) -> bool {

        for i in CONNECTION_DIRECTIONS {
            let mut connections = self.opp_pieces;

            for _ in 0..3 {
                connections &= connections >> i;
            }

            if connections != 0 {
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
        let next_move = self.open_row(col);

        if (next_move & IS_LEGAL) != 0 {
            Some(Self {
                curr_pieces: self.opp_pieces,
                opp_pieces: self.updated_pieces(next_move),
                height_map: self.update_height_map(next_move),
                moves_made: self.moves_made + 1,
            })
        } else {
            None
        }
    }

    fn next_states(&self) -> Vec<Self> {
        let mut next_states = vec![];

        for col in DEFAULT_MOVE_ORDER {
            if let Some(next_state) = self.play_move(col) {
                next_states.push(next_state);
            }
        }

        next_states
    }

    fn encode(board: &Vec<String>) -> Self {

        let mut game_state = Self::allocate();

        for c in 0..COLS {
            let mut cell = 1 << (c * (ROWS + 1));

            for r in 0..ROWS {
                let row = board[ROWS - 1 - r].as_bytes();

                if c >= row.len() {
                    break
                }

                let piece = row[c] as char;
                
                if piece == EMPTY_CELL {
                    break;
                }

                if piece == FIRST_PLAYER {
                    game_state.curr_pieces |= cell;
                } else if piece == SECOND_PLAYER {
                    game_state.opp_pieces |= cell;
                }

                game_state.moves_made += 1;

                cell <<= 1;
            }

            game_state.height_map |= cell;
        }

        if (game_state.moves_made & 1) == 1 {
            let temp = game_state.curr_pieces;
            game_state.curr_pieces = game_state.opp_pieces;
            game_state.opp_pieces = temp;
        }

        game_state
    }

    fn decode(&self) -> String {
        let mut board_str = String::new();
        
        let curr_piece = if (self.moves_made & 1) == 0 {
            FIRST_PLAYER
        } else {
            SECOND_PLAYER
        };

        let opp_piece = if (self.moves_made & 1) == 0 {
            SECOND_PLAYER
        } else {
            FIRST_PLAYER
        };

        for r in (0..ROWS).rev() {
            let mut cell = 1 << r;

            for _ in 0..COLS {
                if (self.curr_pieces & cell) != 0 {
                    board_str.push(curr_piece);
                } else if (self.opp_pieces & cell) != 0 {
                    board_str.push(opp_piece);
                } else {
                    board_str.push(EMPTY_CELL);
                }

                cell <<= ROWS + 1;
            }

            board_str.push('\n');
        }

        board_str
    }
}

impl StateBitboard {

    fn allocate() -> Self {
        Self {
            curr_pieces: 0,
            opp_pieces: 0,
            height_map: 0,
            moves_made: 0,
        }
    }

    fn open_row(&self, col: usize) -> u64 {
        self.height_map & (COL_MASK << (col * (ROWS + 1)))
    }

    fn updated_pieces(&self, next_move: u64) -> u64 {
        self.curr_pieces | next_move
    }

    fn update_height_map(&self, next_move: u64) -> u64 {
        self.height_map + next_move
    }
}

impl fmt::Display for StateBitboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.decode())
    }
}
