use std::cmp::{max};
use crate::state::State;

const LOSS: i32 = -1;
const DRAW: i32 = 0;
const WIN: i32 = 1;

pub struct EvaluatePositionReturn {
    pub eval: i32,
    pub positions_evaluated: usize,
}

pub fn evaluate_position_rec(
    state: State,
    mut alpha: i32,
    beta: i32,
    positions_evaluated: &mut usize,
) -> i32 {

    *positions_evaluated += 1;
    
    if state.board_full() {
        return DRAW;
    }

    let next_states = state.next_states();

    for next_state in &next_states {

        if next_state.is_win() {
            return state.max_eval();
        }
    }

    for next_state in next_states {

        let eval = -evaluate_position_rec(
            next_state,
            -beta,
            -alpha,
            positions_evaluated,
        );

        alpha = max(alpha, eval);

        if alpha >= beta {
            return alpha;
        }
    }

    alpha
}

pub fn evaluate_position(state: State) -> EvaluatePositionReturn {
    let mut positions = 0;
    let eval = evaluate_position_rec(state, LOSS, WIN, &mut positions);

    EvaluatePositionReturn {
        eval,
        positions_evaluated: positions
    }
}
