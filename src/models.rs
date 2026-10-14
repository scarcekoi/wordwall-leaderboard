struct Wordwall {
    score: u8,
    time: u16,
    name: String,
    ids: Id,
}

struct Id {
    activity: u32,
    template: u8,
}
