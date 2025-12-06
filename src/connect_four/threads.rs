use std::cmp::{max, min};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::JoinHandle;
use dashmap::DashMap;
use crate::connect_four::solver_util::{EvaluatePositionReturn, DRAW, WORST_EVAL, BEST_EVAL};
use crate::connect_four::naive;
use crate::connect_four::state::State;


const MAX_CACHED_DEPTH: usize = 35;

struct ThreadContext<S: State> {
    states_evaluated: usize,
    terminate_signal: Arc<AtomicBool>,
    cache: Arc<SharedStateCache<S>>,
}

struct HelperThreadHandler {
    join_handle: JoinHandle<usize>,
    terminate_signal: Arc<AtomicBool>,
}

struct SharedStateCache<S: State> {
    alpha_cache: DashMap<S, i32>,
    beta_cache: DashMap<S, i32>,
}

impl<S: State> SharedStateCache<S> {
    fn new() -> Self {
        Self {
            alpha_cache: DashMap::new(),
            beta_cache: DashMap::new(),
        }
    }

    fn insert_alpha_bound(&self, state: S, bound: i32) {
        self.alpha_cache.insert(state, bound);
    }

    fn insert_beta_bound(&self, state: S, bound: i32) {
        self.beta_cache.insert(state, bound);
    }

    fn fetch_alpha_bound(&self, state: &S) -> i32 {
        *self.alpha_cache.get(state).as_deref().unwrap_or(&WORST_EVAL)
    }

    fn fetch_beta_bound(&self, state: &S) -> i32 {
        *self.beta_cache.get(state).as_deref().unwrap_or(&BEST_EVAL)
    }
}

fn evaluate_position_rec<S: State>(
    state: S,
    mut alpha: i32,
    mut beta: i32,
    ctx: &mut ThreadContext<S>,
) -> Option<i32> {

    if ctx.terminate_signal.load(Ordering::Relaxed) {
        return None
    }

    if state.moves_made() > MAX_CACHED_DEPTH {
        return Some(naive::evaluate_position_rec(state, alpha, beta, &mut ctx.states_evaluated));
    }

    ctx.states_evaluated += 1;

    if state.board_full() {
        return Some(DRAW);
    }

    alpha = max(alpha, ctx.cache.fetch_alpha_bound(&state));
    beta = min(beta, ctx.cache.fetch_beta_bound(&state));

    let next_states = state.next_states();

    for next_state in &next_states {

        if next_state.is_win() {
            return Some(state.max_eval());
        }

        alpha = max(alpha, -ctx.cache.fetch_beta_bound(next_state));
    }

    for next_state in next_states {

        let eval = -evaluate_position_rec(
            next_state,
            -beta,
            -alpha,
            ctx,
        )?;

        alpha = max(alpha, eval);

        if alpha >= beta {
            ctx.cache.insert_alpha_bound(state, alpha);
            return Some(alpha);
        }
    }

    ctx.cache.insert_beta_bound(state, alpha);
    Some(alpha)
}

pub fn evaluate_position<S: State>(state: S) -> EvaluatePositionReturn {

    let cache = Arc::new(SharedStateCache::new());
    let mut handlers = vec![];

    for next_state in state.next_states() {
        let terminate_signal = Arc::new(AtomicBool::new(false));

        let mut ctx = ThreadContext {
            terminate_signal: terminate_signal.clone(),
            states_evaluated: 0,
            cache: cache.clone()
        };

        let handle = thread::spawn(move || {
            const HELPER_THREAD_BETA: i32 = 1; // this significantly affects performance
            evaluate_position_rec(next_state, -HELPER_THREAD_BETA, HELPER_THREAD_BETA, &mut ctx);
            ctx.states_evaluated
        });

        handlers.push(HelperThreadHandler {
            terminate_signal,
            join_handle: handle,
        })
    }

    let mut master_thread_ctx = ThreadContext {
        terminate_signal: Arc::new(AtomicBool::new(false)),
        states_evaluated: 0,
        cache
    };

    let eval = evaluate_position_rec(state, WORST_EVAL, BEST_EVAL, &mut master_thread_ctx).unwrap();

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
