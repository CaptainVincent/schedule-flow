use std::time::{SystemTime, UNIX_EPOCH};

use http_req::request;
use schedule_flows::schedule_cron_job;
use serde_derive::{Deserialize, Serialize};

use slack_flows::send_message_to_channel;
use flowsnet_platform_sdk::logger;

#[no_mangle]
pub fn run() {
    logger::init();
    let keyword = std::env::var("KEYWORD").unwrap();
    println!("123");
    schedule_cron_job(String::from("50 * * * *"), keyword, callback);
}

fn callback(keyword: Vec<u8>) {
}
