use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{http, Filter, Reply, Rejection};

use crate::Store;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    name: String,
    description: String,
    pub id: Option<String>
}


/// Returns the list of all events.
/// 
async fn get_list(
    store: Store
    ) -> Result<impl Reply, Rejection> {
        let r = store.events_list.read();
        Ok(warp::reply::json(&*r))
}


/// This middleware creates `Event` payloads from POST request bodies.
/// 
pub fn post_json() -> impl Filter<Extract = (Event,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


/// Creates an event (lol).
/// 
async fn create_event(
    entry: Event,
    store: Store
    ) -> Result<impl Reply, Rejection> {
        // Checking for existing event
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.name == entry.name).unwrap_or(usize::MAX);
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                "",
                http::StatusCode::ALREADY_REPORTED,
            ));
        }

        let event_id = Uuid::new_v4().to_string();
        let mut write_lock = store.events_list.write();
        write_lock.push(Event { name: entry.name, description: entry.description, id: Some(event_id.clone()) });

        // Create associated maps
        let mut maps_write_lock = store.maps_list.write();
        maps_write_lock.insert(event_id, [].to_vec());

        Ok(warp::reply::with_status(
            "",
            http::StatusCode::CREATED,
        ))
}


/// Returns all event-associated routes:
///     * one route to list all events;
///     * one route to create events.
/// 
pub fn get_routes(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let get_all_events = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list);

    let event_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(create_event);

    get_all_events.or(event_creation_route)
}
