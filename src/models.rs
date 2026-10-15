/// The info about the leaderboard entry.
pub struct Wordwall {
    /// The score of the leaderboard entry.
    ///
    /// Wordwall has a max accepted score of `2147483646`.
    pub score: u32,
    /// The time of the leaderboard entry in milliseconds.
    pub time: u32,
    pub name: String,
    pub ids: Ids,
}

/// The IDs needed for creating the leaderboard entry.
pub struct Ids {
    pub activity: u32,
    pub template: u8,
}
