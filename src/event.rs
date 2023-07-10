use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{http, Filter, hyper::StatusCode};

use crate::Store;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    name: String,
    pub id: Option<String>,
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.events_list.read();
        Ok(warp::reply::json(&*r))
}

pub async fn create_event(
    entry: Event,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Checking for existing entry
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.name == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                "",
                http::StatusCode::ALREADY_REPORTED,
            ));
        }

        let mut write_lock = store.events_list.write();
        write_lock.push(Event { name: entry.name, id: Some(Uuid::new_v4().to_string()) });

        Ok(warp::reply::with_status(
            "",
            http::StatusCode::CREATED,
        ))
}

pub fn post_json() -> impl Filter<Extract = (Event,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn get_event(
    event_id: String,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    // Checking for existing entry
    let events: Vec<Event> = store.events_list.read().to_vec();
    let index = events.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or_else(|| { usize::MAX });
    if index != usize::MAX {
        let event = &events[index];
        return Ok(warp::reply::with_status(
            warp::reply::json(&event),
            StatusCode::OK,
        ));
    }
    return Ok(warp::reply::with_status(
        warp::reply::json(&"{}"),
        StatusCode::NOT_FOUND,
    ));
}