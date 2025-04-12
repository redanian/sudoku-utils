/// Difficulty levels of sudoku solving strategies.
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    /// All enum values.
    pub const VALUES: [Difficulty; 3] = [Difficulty::Easy, Difficulty::Medium, Difficulty::Hard];
}
