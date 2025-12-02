use std::time::Instant;
use crate::engine::{evaluate_position};
use crate::state::State;

mod min;
mod engine;
mod state;
mod piece;

fn main() {

    // let board = vec![
    //     "   O   ",
    //     " O X X ",
    //     " XXOOX ",
    //     " OOXXO ",
    //     " XXOXO ",
    //     "OXOXXOO",
    // ];

    // let board = vec![
    //     "       ",
    //     "       ",
    //     "       ",
    //     "XX     ",
    //     "OOX    ",
    //     "XOOX   ",
    // ];

    let board = vec![
        "   O   ",
        "   X   ",
        "   O X ",
        "   X O ",
        "  XO O ",
        "XXOXOX ",
    ];

    let state = State::encode(&board);

    // for s in state.next_states() {
    //     println!("{s} {}", s.is_win());
    // }
    let mut pos = 0;
    let start = Instant::now();

    println!("{}", evaluate_position(state).positions_evaluated);
    // println!("{pos}");
    println!("{:?}", start.elapsed());
}

