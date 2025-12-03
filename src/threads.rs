use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use dashmap::DashMap;
use crate::naive;
use crate::state::State;
use crate::evaluate_position_util::{EvaluatePositionReturn, DRAW, LOSS, WIN};

struct ThreadLocalContext {
    states_evaluated: usize,
    terminate_signal: Arc<AtomicBool>,
}

struct HelperThreadHandler {
    join_handle: JoinHandle<usize>,
    terminate_signal: Arc<AtomicBool>,
}

struct SharedStateCache {
    alpha_cache: DashMap<State, i32>,
    beta_cache: DashMap<State, i32>,
}

impl SharedStateCache {
    fn new() -> Self {
        Self {
            alpha_cache: DashMap::new(),
            beta_cache: DashMap::new(),
        }
    }

    fn insert_alpha_bound(&self, state: State, bound: i32) {
        self.alpha_cache.insert(state, bound);
    }

    fn insert_beta_bound(&self, state: State, bound: i32) {
        self.beta_cache.insert(state, bound);
    }

    fn fetch_alpha_bound(&self, state: &State) -> i32 {
        *self.alpha_cache.get(state).as_deref().unwrap_or(&LOSS)
    }

    fn fetch_beta_bound(&self, state: &State) -> i32 {
        *self.beta_cache.get(state).as_deref().unwrap_or(&WIN)
    }
}

const MAX_CACHED_DEPTH: usize = 35;

fn evaluate_position_rec(
    state: State,
    mut alpha: i32,
    mut beta: i32,
    thread_local_context: &mut ThreadLocalContext,
    cache: Arc<SharedStateCache>
) -> Option<i32> {

    if thread_local_context.terminate_signal.load(Ordering::Relaxed) {
        return None
    }

    if state.moves_made() > MAX_CACHED_DEPTH {
        return Some(naive::evaluate_position_rec(state, alpha, beta, &mut thread_local_context.states_evaluated));
    }

    thread_local_context.states_evaluated += 1;

    if state.board_full() {
        return Some(DRAW);
    }

    alpha = max(alpha, cache.fetch_alpha_bound(&state));
    beta = min(beta, cache.fetch_beta_bound(&state));

    let next_states = state.next_states();

    for next_state in &next_states {

        if next_state.is_win() {
            return Some(state.max_eval());
        }
    }

    for next_state in next_states {

        let eval = -evaluate_position_rec(
            next_state,
            -beta,
            -alpha,
            thread_local_context,
            cache.clone(),
        )?;

        alpha = max(alpha, eval);

        if alpha >= beta {
            cache.insert_alpha_bound(state, alpha);
            return Some(alpha);
        }
    }

    cache.insert_beta_bound(state, alpha);
    Some(alpha)
}

pub fn evaluate_position(state: State) -> EvaluatePositionReturn {

    let cache = Arc::new(SharedStateCache::new());
    let mut handlers = vec![];

    for next_state in state.next_states() {
        let terminate_signal = Arc::new(AtomicBool::new(false));
        let cache_clone = cache.clone();

        let mut ctx = ThreadLocalContext {
            terminate_signal: terminate_signal.clone(),
            states_evaluated: 0
        };

        let handle = thread::spawn(move || {
            evaluate_position_rec(next_state, LOSS, WIN, &mut ctx, cache_clone);
            ctx.states_evaluated
        });

        handlers.push(HelperThreadHandler {
            terminate_signal,
            join_handle: handle,
        })
    }

    let mut master_thread_ctx = ThreadLocalContext {
        terminate_signal: Arc::new(AtomicBool::new(false)),
        states_evaluated: 0
    };

    let eval = evaluate_position_rec(state, LOSS, WIN, &mut master_thread_ctx, cache).unwrap();

    for handler in &handlers {
        handler.terminate_signal.store(true, Ordering::Relaxed);
    }

    let mut states_evaluated = master_thread_ctx.states_evaluated;

    for handler in handlers {
        let a = handler.join_handle.join();
        states_evaluated += a.unwrap()
    }

    EvaluatePositionReturn::new(eval, states_evaluated)
}
