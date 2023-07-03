use std::{thread, time::Duration, fs::File};
use std::io::prelude::*;

use crate::Store;

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