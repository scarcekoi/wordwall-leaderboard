pub struct Wordwall {
    /// The score of the leaderboard entry.
    pub score: u8,
    /// The time of the leaderboard entry in milliseconds.
    pub time: u32,
    pub name: String,
    pub ids: Ids,
}

pub struct Ids {
    pub activity: u32,
    pub template: u8,
}
