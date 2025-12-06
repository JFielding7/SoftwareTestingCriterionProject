use std::cmp::{max, min};
use std::collections::HashMap;
use crate::connect_four::solver_util::{EvaluatePositionReturn, DRAW, WORST_EVAL, BEST_EVAL};
use crate::connect_four::naive;
use crate::connect_four::state::State;

const MAX_CACHED_DEPTH: usize = 42;

struct GlobalState<S: State> {
    cache: StateCache<S>,
    positions_evaluated: usize
}

impl<S: State> GlobalState<S> {
    fn new() -> Self {
        Self {
            cache: StateCache::new(),
            positions_evaluated: 0
        }
    }
}

struct StateCache<S: State> {
    alpha_cache: HashMap<S, i32>,
    beta_cache: HashMap<S, i32>,
}

impl<S: State> StateCache<S> {
    fn new() -> Self {
        Self {
            alpha_cache: HashMap::new(),
            beta_cache: HashMap::new(),
        }
    }

    fn insert_alpha_bound(&mut self, state: S, bound: i32) {
        self.alpha_cache.insert(state, bound);
    }

    fn insert_beta_bound(&mut self, state: S, bound: i32) {
        self.beta_cache.insert(state, bound);
    }

    fn fetch_alpha_bound(&mut self, state: &S) -> i32 {
        *self.alpha_cache.get(state).unwrap_or(&WORST_EVAL)
    }

    fn fetch_beta_bound(&mut self, state: &S) -> i32 {
        *self.beta_cache.get(state).unwrap_or(&BEST_EVAL)
    }
}

fn evaluate_position_rec<S: State>(
    state: S,
    mut alpha: i32,
    mut beta: i32,
    global_state: &mut GlobalState<S>
) -> i32 {

    if state.moves_made() > MAX_CACHED_DEPTH {
        return naive::evaluate_position_rec(state, alpha, beta, &mut global_state.positions_evaluated);
    }

    global_state.positions_evaluated += 1;

    if state.board_full() {
        return DRAW;
    }

    alpha = max(alpha, global_state.cache.fetch_alpha_bound(&state));
    beta = min(beta, global_state.cache.fetch_beta_bound(&state));

    let next_states = state.next_states();

    for next_state in &next_states {

        if next_state.is_win() {
            return state.max_eval();
        }

        alpha = max(alpha, -global_state.cache.fetch_beta_bound(next_state));
    }

    for next_state in next_states {

        let eval = -evaluate_position_rec(
            next_state,
            -beta,
            -alpha,
            global_state,
        );

        alpha = max(alpha, eval);

        if alpha >= beta {
            global_state.cache.insert_alpha_bound(state, alpha);
            return alpha;
        }
    }

    global_state.cache.insert_beta_bound(state, alpha);
    alpha
}

pub fn evaluate_position<S: State>(state: S) -> EvaluatePositionReturn {

    let mut global_state = GlobalState::new();

    let eval = evaluate_position_rec(state, WORST_EVAL, BEST_EVAL, &mut global_state);

    EvaluatePositionReturn::new(eval, global_state.positions_evaluated)
}

pub fn optimal_next_state<S: State>(state: S) -> S {
    let mut global_state = GlobalState::new();
    let mut max_eval = WORST_EVAL;
    let mut optimal_state = state.clone();

    for next_state in state.next_states() {
        let eval = evaluate_position_rec(next_state.clone(), max_eval, BEST_EVAL, &mut global_state);

        if eval > max_eval {
            max_eval = eval;
            optimal_state = next_state.clone();
        }
    }

    optimal_state
}
