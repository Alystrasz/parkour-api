use warp::{http, Filter};
use parking_lot::RwLock;
use std::{sync::Arc, fs::File};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

type ScoreEntries = Vec<ScoreEntry>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ScoreEntry {
    name: String,
    time: f32,
}

#[derive(Clone)]
struct Store {
  scores_list: Arc<RwLock<ScoreEntries>>
}

impl Store {
    fn new() -> Self {
        Store {
            scores_list: Arc::new(RwLock::new(Vec::new()))
        }
    }
}
async fn update_scores_list(
    entry: ScoreEntry,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Checking for existing entry
        let scores = store.scores_list.read().to_vec();
        let index = scores.iter().position(|e| e.name == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            let existing_entry = &scores[index];
            // If existing entry is better than new entry, we keep the new entry
            if entry.time >= existing_entry.time {
                return Ok(warp::reply::with_status(
                    "",
                    http::StatusCode::ALREADY_REPORTED,
                ));
            } 
            // Else, we remove the existing entry
            else {
                store.scores_list.write().remove(index);
            }
        }
        
        let mut write_lock = store.scores_list.write();
        write_lock.push(ScoreEntry { name: entry.name, time: entry.time });

        // Sort list by times
        write_lock.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
        
        Ok(warp::reply::with_status(
            "",
            http::StatusCode::CREATED,
        ))
}

async fn get_scores_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.scores_list.read();
        Ok(warp::reply::json(&*r))
}

fn post_json() -> impl Filter<Extract = (ScoreEntry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[tokio::main]
async fn main() {
    // Secret key
    let mut file = match File::open(".env.key") {
        Ok(file) => file,
        Err(err) => {
            println!("Error: \".env.key\" secret file does not exist [{}].", err);
            std::process::exit(1);
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            println!("Error: Failed reading secret file [{}].", err);
            std::process::exit(2);
        }
    }

    // Authentication control
    let header_value = Box::leak(data.into_boxed_str());
    let accept_requests = warp::header::exact("authentication", header_value);

    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());


    // Routes
    let get_scores = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_scores_list);

    let add_scores = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(update_scores_list);

    let routes = add_scores.or(get_scores);

    warp::serve(accept_requests.and(routes))
        .run(([127, 0, 0, 1], 3030))
        .await;
}