use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{http, Filter, hyper::StatusCode};

use crate::{Store, event::Event};

pub type Maps = HashMap<String, Vec<Map>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Map {
    map_name: String,
    pub id: Option<String>,
    pub perks: Option<HashMap<String, String>>
}

pub async fn get_list(
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

    let read_lock = store.maps_list.read();
    let maps = read_lock.get(&event_id).unwrap();
    return Ok(warp::reply::with_status(
        warp::reply::json(&maps),
        http::StatusCode::OK,
    ));
}

pub async fn create_map(
    event_id: String,
    entry: Map,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Check if the event exists
        let events: Vec<Event> = store.events_list.read().to_vec();
        let index = events.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or_else(|| { usize::MAX });
        if index == usize::MAX {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"error\": \"Event not found.\"}"),
                StatusCode::NOT_FOUND,
            ));
        }

        // Checking for existing map
        let mut maps: Vec<Map> = store.maps_list.read().get(&event_id).unwrap().to_vec();
        let index = maps.iter().position(|e| e.map_name == entry.map_name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"error\": \"Map already exists.\"}"),
                http::StatusCode::ALREADY_REPORTED,
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
            http::StatusCode::CREATED,
        ))
}

pub fn post_json() -> impl Filter<Extract = (Map,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
