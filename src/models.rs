/// The info about the leaderboard entry.
pub struct Wordwall {
    /// The score of the leaderboard entry.
    pub score: u64,
    /// The time of the leaderboard entry in milliseconds.
    pub time: u64,
    pub name: String,
    pub ids: Ids,
}

/// The IDs needed for creating the leaderboard entry.
pub struct Ids {
    pub activity: u32,
    pub template: u8,
}
