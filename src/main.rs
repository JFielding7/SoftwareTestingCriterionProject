use std::io;
use crate::connect_four::state_file::generate_state_file;

mod connect_four;

fn main() -> io::Result<()> {
    // for i in 16..=40 {
    //     generate_state_file(i)?;
    // }

    Ok(())
}
