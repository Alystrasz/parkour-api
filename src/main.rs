use warp::{http, Filter};
use parking_lot::RwLock;
use std::{sync::Arc, fs::File};
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

type ScoreEntries = Vec<(String, f32)>;

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
        let index = scores.iter().position(|e| e.0 == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            let existing_entry = &scores[index];
            // If existing entry is better than new entry, we keep the new entry
            if entry.time >= existing_entry.1 {
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
        
        store.scores_list.write().push((entry.name, entry.time));

        // TODO sort list by times
        
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
    let mut file = File::open(".env.key").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data).expect("Error while reading file");

    // Authentication control
    let header_value = Box::leak(data.into_boxed_str());
    let accept_requests = warp::header::exact("authentication", header_value);

    // Routes
    let store = Store::new();
    let store_filter = warp::any().map(move || store.clone());

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