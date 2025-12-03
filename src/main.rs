use std::time::Instant;
use crate::example_min::multi_threaded_min_fast;
use crate::naive::{evaluate_position};
use crate::state::State;

mod example_min;
mod naive;
mod state;
mod piece;
mod evaluate_position_util;

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

    // let board = vec![
    //     "   O   ",
    //     "   X   ",
    //     "   O X ",
    //     "   X O ",
    //     "  XO O ",
    //     "XXOXOX ",
    // ];
    //
    // let state = State::encode(&board);
    //
    // // for s in state.next_states() {
    // //     println!("{s} {}", s.is_win());
    // // }
    // let mut pos = 0;
    // let start = Instant::now();
    //
    // println!("{}", evaluate_position(state).positions_evaluated);
    // // println!("{pos}");
    // println!("{:?}", start.elapsed());

    let v: Vec<i32> = (0..10000000).collect();
    let start = Instant::now();

    let min = multi_threaded_min_fast(&v, 8);

    println!("{:?}", start.elapsed());
}

