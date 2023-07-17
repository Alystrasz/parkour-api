use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use warp::{http, Filter, hyper::StatusCode};

use crate::Store;

pub type Maps = Vec<Map>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Map {
    name: String,
    pub id: Option<String>,
    pub perks: Option<HashMap<String, String>>
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.maps_list.read();
        Ok(warp::reply::json(&*r))
}

pub async fn create_map(
    entry: Map,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        // Checking for existing entry
        let maps: Vec<Map> = store.maps_list.read().to_vec();
        let index = maps.iter().position(|e| e.name == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                "",
                http::StatusCode::ALREADY_REPORTED,
            ));
        }

        let event_id = Uuid::new_v4().to_string();
        let mut write_lock = store.maps_list.write();
        write_lock.push(Map { name: entry.name, id: Some(event_id.clone()), perks: Some(entry.perks).unwrap_or_else(|| { Some(HashMap::new()) }) });

        let mut scores_write_lock = store.scores_list.write();
        scores_write_lock.insert(event_id, [].to_vec());

        Ok(warp::reply::with_status(
            "",
            http::StatusCode::CREATED,
        ))
}

pub fn post_json() -> impl Filter<Extract = (Map,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn get_map(
    event_id: String,
    store: Store
) -> Result<impl warp::Reply, warp::Rejection> {
    // Checking for existing entry
    let maps: Vec<Map> = store.maps_list.read().to_vec();
    let index = maps.iter().position(|e| e.id.clone().unwrap() == event_id).unwrap_or_else(|| { usize::MAX });
    if index != usize::MAX {
        let event = &maps[index];
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