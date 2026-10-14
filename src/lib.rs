//! `wordwall_leaderboard` is a tool for creating leaderboard entries on [Wordwall](https://wordwall.net).
//!
//! # Examples
//!
//! ```
//! let ids = Ids {
//!     activity: 2470189,
//!     template: 2,
//! };
//!
//! let ww = Wordwall {
//!     score: 60,
//!     time: 10000,
//!     name: "John Smith".to_string(),
//!     ids,
//! };
//!
//! leaderboard_entry_constructor(ww);
//! ```

use curl::easy::Easy;
use std::io::Read;

use crate::models::Wordwall;

pub mod models;

pub fn build_leaderboard_entry_payload(wdata: Wordwall) -> (String, Easy) {
    let data: String = "score=".to_string()
        + wdata.score.to_string().as_str()
        + "&time="
        + fix_time(wdata.time).as_str()
        + "&name="
        + wdata.name.as_str()
        + "&activityId="
        + wdata.ids.activity.to_string().as_str()
        + "&templateId="
        + wdata.ids.template.to_string().as_str();

    let bdata = data.as_bytes();

    let mut easy = Easy::new();
    easy.url("https://wordwall.net/leaderboardajax/addentry")
        .unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(bdata.len() as u64).unwrap();

    (data, easy)
}

pub fn post_leaderboard_entry(data: String, mut easy: Easy) {
    let mut data = data.as_bytes();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();
}

/// Converts a [`u32`] to a [`String`] that has leading 0s to the 5-character mark.
pub fn fix_time(time: u64) -> String {
    if time > 99999 {
        eprintln!("time is too high");
    }

    format!("{:0width$}", time, width = 5)
}
