use std::cmp::{max, min};
use std::collections::HashMap;
use crate::naive;
use crate::naive::EvaluatePositionReturn;
use crate::state::State;

const LOSS: i32 = -1;
const DRAW: i32 = 0;
const WIN: i32 = 1;

struct GlobalState {
    cache: StateCache,
    positions_evaluated: usize
}

impl GlobalState {
    fn new() -> Self {
        Self {
            cache: StateCache::new(),
            positions_evaluated: 0
        }
    }
}

struct StateCache {
    alpha_cache: HashMap<State, i32>,
    beta_cache: HashMap<State, i32>,
}

impl StateCache {
    fn new() -> Self {
        Self {
            alpha_cache: HashMap::new(),
            beta_cache: HashMap::new(),
        }
    }

    fn insert_alpha_bound(&mut self, state: State, bound: i32) {
        self.alpha_cache.insert(state, bound);
    }

    fn insert_beta_bound(&mut self, state: State, bound: i32) {
        self.beta_cache.insert(state, bound);
    }

    fn fetch_alpha_bound(&mut self, state: &State) -> i32 {
        *self.alpha_cache.get(state).unwrap_or(&LOSS)
    }

    fn fetch_beta_bound(&mut self, state: &State) -> i32 {
        *self.beta_cache.get(state).unwrap_or(&WIN)
    }
}

const MAX_CACHED_DEPTH: usize = 35;

fn evaluate_position_rec(
    state: State,
    mut alpha: i32,
    mut beta: i32,
    global_state: &mut GlobalState
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

pub fn evaluate_position(state: State) -> EvaluatePositionReturn {

    let mut global_state = GlobalState::new();

    let eval = evaluate_position_rec(state, LOSS, WIN, &mut global_state);

    EvaluatePositionReturn {
        eval,
        positions_evaluated: global_state.positions_evaluated
    }
}
