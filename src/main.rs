mod persistence;
pub mod log;
pub mod map;
mod event;
mod scores;

use event::Events;
use map::Maps;
use persistence::{start_save_cron, load_state};
use warp::Filter;
use parking_lot::RwLock;
use std::{env, sync::Arc, collections::HashMap};


#[derive(Clone)]
pub struct Store {
  events_list: Arc<RwLock<Events>>,  
  scores_list: Arc<RwLock<scores::ScoreEntries>>,
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

    // Routes
    let map_routes = map::get_routes(store.clone());
    let event_routes = event::get_routes(store.clone());
    let score_routes = scores::get_routes(store.clone());
    let routes = event_routes.or(map_routes).or(score_routes);

    warp::serve(accept_requests.and(routes))
        .run(([0, 0, 0, 0], 3030))
        .await;
}