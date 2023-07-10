use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;

use crate::{Store, ScoreEntries, log};

pub fn start_save_cron(store: Store) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(15 * 60));
            let scores = store.scores_list.read().clone();

            let mut buffer = match File::create("list.json") {
                Ok(file) => file,
                Err(err) => {
                    log::error(&format!("\".list.json\" file could not be created [{}].", err));
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
        }
    });
}

pub fn load_state(store: Store) {
    let mut file = match File::open("list.json") {
        Ok(file) => file,
        Err(_) => {
            log::info("\"list.json\" file does not exist, initializing scores list as empty.");
            return;
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            log::error(&format!("Failed reading \"list.json\" file [{}].", err));
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
    log::info("Loaded scores list from \"list.json\" file.");
}