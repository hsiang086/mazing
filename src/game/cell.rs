use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Cell {
    Wall,
    Path,
    Solution, // Marks the solution path
}
