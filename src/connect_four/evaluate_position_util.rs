pub const LOSS: i32 = -1;
pub const DRAW: i32 = 0;
pub const WIN: i32 = 1;

pub struct EvaluatePositionReturn {
    pub eval: i32,
    pub states_evaluated: usize,
}

impl EvaluatePositionReturn {
    pub fn new(eval: i32, states_evaluated: usize) -> EvaluatePositionReturn {
        EvaluatePositionReturn {
            eval,
            states_evaluated
        }
    }
}
