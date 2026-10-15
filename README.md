`wordwall-leaderboard` is a tool for creating leaderboard entries on [Wordwall](https://wordwall.net).

# Examples

```rust
use wordwall_leaderboard::{
    build_leaderboard_entry_payload,
    models::{Ids, Wordwall},
    post_leaderboard_entry,
};

fn main() {
    let ids = Ids {
        activity: 2470189,
        template: 2,
    };

    let ww = Wordwall {
        score: u32::MAX - 2147483649,
        time: 10000,
        name: "John Smith".to_string(),
        ids,
    };

    let (data, easy) = build_leaderboard_entry_payload(ww);

    post_leaderboard_entry(data, easy);
}
```
