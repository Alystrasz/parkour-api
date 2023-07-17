use std::env;
use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;

use crate::map::Maps;
use crate::{Store, ScoreEntries, log};

const MAPS_FILE: &str = "maps.json";
const SCORES_FILE: &str = "scores.json";

pub fn start_save_cron(store: Store) {
    let cron_interval_minutes: u64 = match env::var("PARKOUR_API_SAVE_TIMER") {
        Ok(s) => {
            log::info(&format!("Timer argument found ({} minutes).", s));
            s.parse::<u64>().unwrap()
        },
        Err(_) => {
            log::info("No timer argument was found, defaulting to 15 minutes.");
            15
        }
    };

    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(cron_interval_minutes * 60));

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

            // Maps
            let maps = store.maps_list.read().clone();
            let mut buffer = match File::create(MAPS_FILE) {
                Ok(file) => file,
                Err(err) => {
                    log::error(&format!("\"{}\" file could not be created [{}].", MAPS_FILE, err));
                    std::process::exit(3);
                }
            };
            let str = match serde_json::to_string(&maps) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed serializing maps list [{}].", err));
                    std::process::exit(3);
                }
            };
            match buffer.write(str.as_bytes()) {
                Ok(str) => str,
                Err(err) => {
                    log::error(&format!("Failed writing maps list to file [{}].", err));
                    std::process::exit(3);
                }
            };
            log::info("Saved maps to local file.");
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

    // Maps
    let mut file = match File::open(MAPS_FILE) {
        Ok(file) => file,
        Err(_) => {
            log::info(&format!("\"{}\" file does not exist, initializing maps list as empty.", MAPS_FILE));
            return;
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            log::error(&format!("Failed reading \"{}\" file [{}].", MAPS_FILE, err));
            std::process::exit(2);
        }
    };
    let serialized: Maps = match serde_json::from_str::<Maps>(&data) {
        Ok(data) => data,
        Err(err) => {
            log::error(&format!("Failed deserializing maps list [{}].", err));
            std::process::exit(2);
        }
    };
    let mut write_lock = store.maps_list.write();
    for value in serialized {
        write_lock.push(value);
    }
    log::info(&format!("Loaded maps list from \"{}\" file.", MAPS_FILE));
}