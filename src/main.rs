use curl::easy::Easy;
use std::io::Read;

use crate::models::{Ids, Wordwall};

mod models;

fn main() {}

fn construct_post_request(wdata: &Wordwall) {
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

    let mut data = data.as_bytes();

    let mut easy = Easy::new();
    easy.url(format!("https://wordwall.net/resource/{}", wdata.ids.activity).as_str())
        .unwrap();
    easy.post(true).unwrap();
    easy.post_field_size(data.len() as u64).unwrap();

    let mut transfer = easy.transfer();
    transfer
        .read_function(|buf| Ok(data.read(buf).unwrap_or(0)))
        .unwrap();
    transfer.perform().unwrap();
}

fn fix_time(time: u32) -> String {
    if time > 99999 {
        eprintln!("time is too high");
    }

    format!("{:0width$}", time, width = 5)
}
