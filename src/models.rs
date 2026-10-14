pub struct Wordwall {
    score: u8,
    time: u16,
    name: String,
    ids: Ids,
}

pub struct Ids {
    activity: u32,
    template: u8,
}
