use std::cmp::{max};
use crate::connect_four::solver_util::{EvaluatePositionReturn, DRAW, WORST_EVAL, BEST_EVAL};
use crate::connect_four::state::State;


pub fn evaluate_position_rec<S: State>(
    state: S,
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

pub fn evaluate_position<S: State>(state: S) -> EvaluatePositionReturn {
    let mut states_evaluated = 0;
    let eval = evaluate_position_rec(state, WORST_EVAL, BEST_EVAL, &mut states_evaluated);

    EvaluatePositionReturn::new(eval, states_evaluated)
}
