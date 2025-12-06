pub const WORST_EVAL: i32 = -18;
pub const DRAW: i32 = 0;
pub const BEST_EVAL: i32 = 18;
pub const ROWS: usize = 6;
pub const COLS: usize = 7;
pub const BOARD_SIZE: usize = ROWS * COLS;
pub const DEFAULT_MOVE_ORDER: [usize; COLS] = [3, 2, 4, 1, 5, 0, 6];
pub const FIRST_PLAYER: char = 'X';
pub const SECOND_PLAYER: char = 'O';
pub const EMPTY_CELL: char = ' ';


pub struct EvaluatePositionReturn {
    pub eval: i32,
    pub states_evaluated: usize,
}

impl EvaluatePositionReturn {
    pub fn new(eval: i32, states_evaluated: usize) -> EvaluatePositionReturn {
        EvaluatePositionReturn {
            eval,
            states_evaluated
        }
    }
}
