use std::{io};
use std::fs::File;
use std::io::BufRead;
use std::path::Path;
use crate::connect_four::cache_strategy::optimal_next_state;
use crate::connect_four::solver_util::ROWS;
use crate::connect_four::state::State;
use crate::connect_four::state_bitboard::StateBitboard;
use std::io::Write;

pub fn read_state_file<S: State>(depth: usize) -> io::Result<Vec<S>> {

    let path_string = format!("positions/positions{depth}");
    let file_path = Path::new(path_string.as_str());
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut states = vec![];
    let mut curr_state = vec![];

    for line_result in reader.lines() {
        let line = line_result?;
        curr_state.push(line);

        if curr_state.len() == ROWS {
            states.push(S::encode(&curr_state));
            curr_state = vec![];
        }
    }

    Ok(states)
}

pub fn generate_state_file(depth: usize) -> io::Result<()> {

    let prev_states: Vec<StateBitboard> = read_state_file(depth - 1)?;
    let mut states = vec![];

    for state in prev_states {
        println!("{state}");

        let optimal_state = optimal_next_state(state);

        states.push(optimal_state);
    }

    let mut path = File::create(format!("positions/positions{depth}"))?;

    for state in states {
        write!(path, "{}", state.decode())?;
    }

    Ok(())
}
