`wordwall-leaderboard` is a tool for creating leaderboard entries on [Wordwall](https://wordwall.net).

# Examples

```rust
let ids = Ids {
    activity: 2470189,
    template: 2,
};

let ww = Wordwall {
    score: 60,
    time: 10000,
    name: "John Smith".to_string(),
    ids: ids,
};

leaderboard_entry_constructor(ww);
```
