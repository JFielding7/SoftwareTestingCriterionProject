use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::Path;
use crate::connect_four::solver_util::ROWS;
use crate::connect_four::state::State;

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
