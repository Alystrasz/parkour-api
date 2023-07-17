mod persistence;
pub mod log;
pub mod map;
mod event;

use event::Events;
use map::Maps;
use persistence::{start_save_cron, load_state};
use warp::Filter;
use parking_lot::RwLock;
use std::{env, sync::Arc, collections::HashMap};
use serde::{Serialize, Deserialize};

type ScoreEntries = HashMap<String, Vec<ScoreEntry>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct ScoreEntry {
    name: String,
    time: f32,
}

#[derive(Clone)]
pub struct Store {
  events_list: Arc<RwLock<Events>>,  
  scores_list: Arc<RwLock<ScoreEntries>>,
  maps_list: Arc<RwLock<Maps>>
}

impl Store {
    fn new() -> Self {
        Store {
            events_list: Arc::new(RwLock::new(Vec::new())),
            scores_list: Arc::new(RwLock::new(HashMap::new())),
            maps_list: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}



#[tokio::main]
async fn main() {
    // Secret key
    let secret = match env::var("PARKOUR_API_SECRET") {
        Ok(s) => s,
        Err(err) => {
            log::error(&format!("No secret was found, exiting [{}].", err));
            std::process::exit(1);
        }
    };

    // Authentication control
    let header_value = Box::leak(secret.into_boxed_str());
    let accept_requests = warp::header::exact("authentication", header_value);

    let store = Store::new();

    // If scores were previously saved to file, restore them
    load_state(store.clone());
    // Scores saving cron
    start_save_cron(store.clone());

    let store_filter = warp::any().map(move || store.clone());


    // Routes

    // Maps
    let map_list_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("maps"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(map::get_list);

    let map_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("maps"))
        .and(warp::path::end())
        .and(map::post_json())
        .and(store_filter.clone())
        .and_then(map::create_map);

    let get_all_events = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(event::get_list);

    let event_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(event::post_json())
        .and(store_filter.clone())
        .and_then(event::create_event);

    let routes = map_creation_route.or(get_all_events).or(event_creation_route).or(map_list_route);

    warp::serve(accept_requests.and(routes))
        .run(([0, 0, 0, 0], 3030))
        .await;
}