use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{http, Filter};

use crate::Store;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    name: String,
    description: String,
    pub id: Option<String>
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.events_list.read();
        Ok(warp::reply::json(&*r))
}

pub fn post_json() -> impl Filter<Extract = (Event,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn create_event(
    entry: Event,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Checking for existing event
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.name == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                "",
                http::StatusCode::ALREADY_REPORTED,
            ));
        }

        let event_id = Uuid::new_v4().to_string();
        let mut write_lock = store.events_list.write();
        write_lock.push(Event { name: entry.name, description: entry.description, id: Some(event_id.clone()) });

        Ok(warp::reply::with_status(
            "",
            http::StatusCode::CREATED,
        ))
}