use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;

use crate::{Store, ScoreEntries};

pub fn start_save_cron(store: Store) {
    thread::spawn(move || {
        loop {
            thread::sleep(Duration::from_secs(5));
            let scores = store.scores_list.read().to_vec();

            let mut buffer = match File::create("list.json") {
                Ok(file) => file,
                Err(err) => {
                    println!("Error: \".list.json\" file could not be created [{}].", err);
                    std::process::exit(3);
                }
            };

            let str = match serde_json::to_string(&scores) {
                Ok(str) => str,
                Err(err) => {
                    println!("Error: failed serializing scores list [{}].", err);
                    std::process::exit(3);
                }
            };
            match buffer.write(str.as_bytes()) {
                Ok(str) => str,
                Err(err) => {
                    println!("Error: failed writing scores list to file [{}].", err);
                    std::process::exit(3);
                }
            };
        }
    });
}

pub fn load_state(store: Store) {
    let mut file = match File::open("list.json") {
        Ok(file) => file,
        Err(_) => {
            println!("Info: \"list.json\" file does not exist, initializing scores list as empty.");
            return;
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            println!("Error: Failed reading \"list.json\" file [{}].", err);
            std::process::exit(2);
        }
    };

    let mut serialized: ScoreEntries = match serde_json::from_str::<ScoreEntries>(&data) {
        Ok(data) => data,
        Err(err) => {
            println!("Error: Failed serializing scores list [{}].", err);
            std::process::exit(2);
        }
    };

    store.scores_list.write().append(&mut serialized);
    println!("Loaded scores list with {} entries from \"list.json\" file.", serialized.len());
}