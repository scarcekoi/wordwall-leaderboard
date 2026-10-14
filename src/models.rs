pub struct Wordwall {
    pub score: u8,
    pub time: u32,
    pub name: String,
    pub ids: Ids,
}

pub struct Ids {
    pub activity: u32,
    pub template: u8,
}
