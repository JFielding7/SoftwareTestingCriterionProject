use std::hash::Hash;

const CURR_PIECE: u8 = b'X';
const OPP_PIECE: u8 = b'O';

pub trait State: Eq + Hash + Sized + Send + Sync + Clone + 'static {

    fn start_state() -> Self;

    fn is_win(&self) -> bool;

    fn board_full(&self) -> bool;

    fn moves_made(&self) -> usize;

    fn max_eval(&self) -> i32;
    fn play_move(&self, col: usize) -> Option<Self>;

    fn next_states(&self) -> Vec<Self>;

    fn encode(board: &Vec<String>) -> Self;

    fn decode(&self) -> String;
}
