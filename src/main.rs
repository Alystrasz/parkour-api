mod persistence;
pub mod log;
pub mod event;

use event::{Events, Event};
use persistence::{start_save_cron, load_state};
use warp::{http, Filter, hyper::StatusCode};
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
  scores_list: Arc<RwLock<ScoreEntries>>,
  events_list: Arc<RwLock<Events>>
}

impl Store {
    fn new() -> Self {
        Store {
            scores_list: Arc::new(RwLock::new(HashMap::new())),
            events_list: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

async fn get_event_scores(
    event_id: String,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    // Checking for existing event
    let events: Vec<Event> = store.events_list.read().to_vec();
    let index = events.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or_else(|| { usize::MAX });
    if index == usize::MAX {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"{\"error\": \"Event not found.\"}"),
            StatusCode::NOT_FOUND,
        ));
    }

    let read_lock = store.scores_list.read();
    let scores = read_lock.get(&event_id).unwrap();
    return Ok(warp::reply::with_status(
        warp::reply::json(&scores),
        http::StatusCode::OK,
    ));
}

async fn create_score_entry(
    event_id: String,
    entry: ScoreEntry,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Check if provided event exists
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.clone().id.unwrap() == event_id).unwrap_or_else(|| { usize::MAX });
        if index == usize::MAX {
            return Ok(warp::reply::with_status(
                "",
                http::StatusCode::NOT_FOUND,
            ))
        }

        // Checking for existing entry
        let mut scores = store.scores_list.read().get(&event_id).unwrap().clone();
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
                scores.remove(index);
            }
        }
        
        // Create new entry
        scores.push(ScoreEntry { name: entry.name, time: entry.time });

        // Sort list by times
        scores.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

        // Restore list
        let mut write_lock = store.scores_list.write();
        write_lock.insert(event_id, scores);
        
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
    let get_scores = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_scores_list);


    // Events
    let events_list_route = warp::get()
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

    let event_details_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(event::get_event);

    let event_scores_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_event_scores);

    let score_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("scores"))
        .and(post_json())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(create_score_entry);

    let routes = get_scores.or(events_list_route).or(event_creation_route).or(event_details_route).or(event_scores_route).or(score_creation_route);

    warp::serve(accept_requests.and(routes))
        .run(([0, 0, 0, 0], 3030))
        .await;
}