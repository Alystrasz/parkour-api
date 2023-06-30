use warp::{http, Filter};
use parking_lot::RwLock;
use std::{sync::Arc, fs};
use serde::{Serialize, Deserialize};

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
        store.scores_list.write().push((entry.name, entry.time));
        
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
    fn read_file_string(filepath: &str) -> Result<String, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(filepath)?;
        Ok(data)
    }
    let secret = read_file_string(".env.key").unwrap();
    dbg!(secret);


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

    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}