extern crate winrt_notification;

use std::{env, thread};
use std::path::Path;
use std::time::Duration;
use log::debug;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

use serde::Deserialize;
use stderrlog::LogLevelNum;


#[derive(Debug, Deserialize)]
struct Config {
    app_ids: Vec<String>,
    interval: Interval,
    messages: Vec<Message>,
}

#[derive(Debug, Deserialize)]
struct Message {
    title: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct Interval {
    start: u64,
    repeat_every: u64,
    std_dev: u64,
}

const DEBUG_FLAG_FILE: &str = "C:\\annoying_jokes.flag";

fn main() {
    stderrlog::new()
        .module(module_path!())
        .verbosity(LogLevelNum::Debug)
        .timestamp(stderrlog::Timestamp::Millisecond)
        .init().unwrap();


    const JSON_STRING: &str = include_str!("messages.json");
    let mut config: Config = serde_json::from_str(JSON_STRING).expect("Failed to load config");

    // Debug mode with faster intervals
    if is_fast_debug_mode() {
        config.interval.start = 0;
        config.interval.repeat_every = 5;
        config.interval.std_dev = 1;
    }


    // wait before sending the first notification
    pause_interval(config.interval.start, config.interval.std_dev);

    let mut rng = rand::thread_rng();
    loop {
        if let Some(app_id) = config.app_ids.choose(&mut rng) {
            if let Some(message) = config.messages.choose(&mut rng) {
                debug!("AppId: \"{}\", title: \"{}\", content: \"{}\"", app_id, message.title, message.content);
                let _ = show_toast(app_id, message);
            }
        }
        pause_interval(config.interval.repeat_every, config.interval.std_dev);
    }
}

fn is_fast_debug_mode() -> bool {
    let args: Vec<String> = env::args().collect();
    let debug_file_exists = Path::new(DEBUG_FLAG_FILE).exists();
    let debug_args_flag = args.len() >= 2 && args[1] == "-t";
    debug_args_flag || debug_file_exists
}

fn pause_interval(mean: u64, std_dev: u64) {
    let v = randomized(mean, std_dev);
    debug!("Waiting for {} seconds", v);
    thread::sleep(Duration::from_secs(v));
}

fn randomized(mean: u64, std_dev: u64) -> u64 {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mean as f64, std_dev as f64).unwrap();
    let value = normal.sample(&mut rng);

    if value < 0.0 {
        0
    } else {
        value as u64
    }
}


fn show_toast(app_id: &str, message: &Message) -> winrt_notification::Result<()> {
    use winrt_notification::{Duration, Sound, Toast};
    Toast::new(app_id)
        .title(&*message.title)
        .text1(&*message.content)
        .sound(Some(Sound::Default))
        .duration(Duration::Short)
        .show()
}