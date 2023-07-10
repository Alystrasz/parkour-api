use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;

use crate::event::Events;
use crate::{Store, ScoreEntries, log};

const EVENTS_FILE: &str = "events.json";
const SCORES_FILE: &str = "scores.json";

pub fn start_save_cron(store: Store) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(15 * 60));

            // Scores
            let scores = store.scores_list.read().clone();
            let mut buffer = match File::create(SCORES_FILE) {
                Ok(file) => file,
                Err(err) => {
                    log::error(&format!("\"{}\" file could not be created [{}].", SCORES_FILE, err));
                    std::process::exit(3);
                }
            };
            let str = match serde_json::to_string(&scores) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed serializing scores list [{}].", err));
                    std::process::exit(3);
                }
            };
            match buffer.write(str.as_bytes()) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed writing scores list to file [{}].", err));
                    std::process::exit(3);
                }
            };
            log::info("Saved scores to local file.");

            // Events
            let events = store.events_list.read().clone();
            let mut buffer = match File::create(EVENTS_FILE) {
                Ok(file) => file,
                Err(err) => {
                    log::error(&format!("\"{}\" file could not be created [{}].", EVENTS_FILE, err));
                    std::process::exit(3);
                }
            };
            let str = match serde_json::to_string(&events) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed serializing events list [{}].", err));
                    std::process::exit(3);
                }
            };
            match buffer.write(str.as_bytes()) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed writing events list to file [{}].", err));
                    std::process::exit(3);
                }
            };
            log::info("Saved events to local file.");
        }
    });
}

pub fn load_state(store: Store) {
    // Scores
    let mut file = match File::open(SCORES_FILE) {
        Ok(file) => file,
        Err(_) => {
            log::info(&format!("\"{}\" file does not exist, initializing scores list as empty.", SCORES_FILE));
            return;
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            log::error(&format!("Failed reading \"{}\" file [{}].", SCORES_FILE, err));
            std::process::exit(2);
        }
    };
    let serialized: ScoreEntries = match serde_json::from_str::<ScoreEntries>(&data) {
        Ok(data) => data,
        Err(err) => {
            log::error(&format!("Failed serializing scores list [{}].", err));
            std::process::exit(2);
        }
    };
    let mut write_lock = store.scores_list.write();
    for (key, value) in serialized {
        write_lock.insert(key, value);
    }
    log::info(&format!("Loaded scores list from \"{}\" file.", SCORES_FILE));

    // Events
    let mut file = match File::open(EVENTS_FILE) {
        Ok(file) => file,
        Err(_) => {
            log::info(&format!("\"{}\" file does not exist, initializing events list as empty.", EVENTS_FILE));
            return;
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            log::error(&format!("Failed reading \"{}\" file [{}].", EVENTS_FILE, err));
            std::process::exit(2);
        }
    };
    let serialized: Events = match serde_json::from_str::<Events>(&data) {
        Ok(data) => data,
        Err(err) => {
            log::error(&format!("Failed deserializing events list [{}].", err));
            std::process::exit(2);
        }
    };
    let mut write_lock = store.events_list.write();
    for value in serialized {
        write_lock.push(value);
    }
    log::info(&format!("Loaded events list from \"{}\" file.", EVENTS_FILE));
}