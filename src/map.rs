use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{Filter, hyper::StatusCode, Reply, Rejection};

use crate::{Store, event::Event};

pub type Maps = HashMap<String, Vec<Map>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Map {
    map_name: String,
    pub id: Option<String>,
    pub perks: Option<HashMap<String, String>>
}


/// Gets the list of maps associated to a given event.
/// 
async fn get_list(
    event_id: String,
    store: Store
    ) -> Result<impl Reply, Rejection> {
    // Checking for existing event
    let events: Vec<Event> = store.events_list.read().to_vec();
    let index = events.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or(usize::MAX);
    if index == usize::MAX {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"{\"error\": \"Event not found.\"}"),
            StatusCode::NOT_FOUND,
        ));
    }

    let read_lock = store.maps_list.read();
    let maps = read_lock.get(&event_id).unwrap();
    Ok(warp::reply::with_status(
        warp::reply::json(&maps),
        StatusCode::OK,
    ))
}


/// Creates a map that's associated to the input event.
/// 
async fn create_map(
    event_id: String,
    entry: Map,
    store: Store
    ) -> Result<impl Reply, Rejection> {
        // Check if the event exists
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or(usize::MAX);
        if index == usize::MAX {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"error\": \"Event not found.\"}"),
                StatusCode::NOT_FOUND,
            ));
        }

        // Checking for existing map
        let mut maps: Vec<Map> = store.maps_list.read().get(&event_id).unwrap().to_vec();
        let index = maps.iter().position(|e| e.map_name == entry.map_name).unwrap_or(usize::MAX);
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"error\": \"Map already exists.\"}"),
                StatusCode::ALREADY_REPORTED,
            ));
        }

        let map_id = Uuid::new_v4().to_string();
        let mut write_lock = store.maps_list.write();
        maps.push(Map { map_name: entry.map_name, id: Some(map_id.clone()), perks: Some(entry.perks).unwrap_or_else(|| { Some(HashMap::new()) }) });
        write_lock.insert(event_id, maps);

        // Create associated scores
        let mut scores_write_lock = store.scores_list.write();
        scores_write_lock.insert(map_id, [].to_vec());

        Ok(warp::reply::with_status(
            warp::reply::json(&"{\"message\": \"Map successfully created.\"}"),
            StatusCode::CREATED,
        ))
}


/// This middleware creates `Map` payloads from POST request bodies.
/// 
pub fn post_json() -> impl Filter<Extract = (Map,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


/// Returns all map-associated routes:
///     * one route to list an event's maps;
///     * one route to create maps on a given event.
/// 
pub fn get_routes(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let map_list_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("maps"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list);

    let map_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("events"))
        .and(warp::path::param())
        .and(warp::path("maps"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(create_map);

    map_list_route.or(map_creation_route)
}
