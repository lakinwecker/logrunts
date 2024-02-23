use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize, Debug)]
enum Level {
    #[serde(rename = "trace")]
    Trace,
    #[serde(rename = "debug")]
    Debug,
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "error")]
    Error,
}

#[derive(Serialize, Deserialize, Debug)]
struct LogMsg {
    environment: String,
    level: Level,
    msg: String,
    name: String,
    version: String,
    #[serde(rename = "trace-id")]
    trace_id: String,
}

fn handle_log_msg(msg: LogMsg) {
    let target = format!("{}/{}@{}", msg.name, msg.environment, msg.version);
    let formatted_msg = if msg.trace_id == "00000000000000000000000000000000" {
        format!("{}", msg.msg)
    } else {
        format!("{} > {}", msg.trace_id, msg.msg)
    };

    match msg.level {
        Level::Trace => trace!(target: &target[..], "{}", formatted_msg),
        Level::Debug => debug!(target: &target[..], "{}", formatted_msg),
        Level::Info => info!(target: &target[..], "{}", formatted_msg),
        Level::Warn => warn!(target: &target[..], "{}", formatted_msg),
        Level::Error => error!(target: &target[..], "{}", formatted_msg),
    }
}

// {
// "environment":"development",
// "level":"info",
// "msg":"Starting 1 workers",
// "name":"analysis-worker",
// "time":"2024-02-12T07:26:38-07:00",
// "trace-id":"00000000000000000000000000000000",
// "version":"1.0"}
fn main() {
    pretty_env_logger::init();
    for line in io::stdin().lines() {
        let line = line.expect("Unable to read line");
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        match serde_json::from_str(&line) {
            Ok(msg) => handle_log_msg(msg),
            Err(_) => println!("{}", line),
        }
    }
}
