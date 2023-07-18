use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, http, Filter, Reply, Rejection};

use crate::Store;

pub type ScoreEntries = HashMap<String, Vec<ScoreEntry>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreEntry {
    name: String,
    time: f32,
}

async fn get_list(
    map_id: String,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {

    let scores_read_lock = store.scores_list.read();
    if !scores_read_lock.contains_key(&map_id) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"{\"message\": \"Map not found.\"}"),
            StatusCode::NOT_FOUND,
        ));
    }

    let scores = scores_read_lock.get(&map_id).unwrap();
    return Ok(warp::reply::with_status(
        warp::reply::json(&scores),
        http::StatusCode::OK,
    ));
}

pub fn get_routes(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let scores_list_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_list);

    return scores_list_route;
}