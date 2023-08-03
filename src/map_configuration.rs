use std::collections::HashMap;
use warp::{hyper::StatusCode, Filter, Reply, Rejection};

use crate::Store;
use serde::{Serialize, Deserialize};


pub type MapConfigurations = HashMap<String, MapConfiguration>;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Line {
    origin: [f64; 3],
    angles: [i64; 3],
    dimensions: [i64; 2],
    trigger: [[f64; 3]; 2]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct LeaderboardSource {
    origin: [f64; 3],
    angles: [i64; 3],
    dimensions: [i64; 2],
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Leaderboard {
    origin: [f64; 3],
    angles: [i64; 3],
    dimensions: [i64; 2],
    source: LeaderboardSource
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Leaderboards {
    local: Leaderboard,
    world: Leaderboard
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StartPosition {
    origin: [f64; 3],
    angles: [i64; 3]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct EndPosition {
    origin: [f64; 3]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapConfiguration {
    startLine: Line,
    finishLine: Line,
    leaderboards: Leaderboards,
    checkpoints: Vec<[f64; 3]>,
    start: StartPosition,
    end: EndPosition,
    ziplines: Vec<[[f64; 3]; 2]>
}


/// This middleware creates `MapConfiguration` payloads from POST request bodies.
/// 
pub fn post_json() -> impl Filter<Extract = (MapConfiguration,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


/// Creates a map configuration, based on its map identifier.
/// 
async fn create_map_configuration(
    map_id: String,
    entry: MapConfiguration,
    store: Store
) -> Result<impl Reply, Rejection> {

    let mut write_lock = store.configurations_list.write();
    write_lock.insert(map_id, entry);

    Ok(warp::reply::with_status(
        warp::reply::json(&"Map configuration updated."),
        StatusCode::CREATED,
    ))
}


/// Returns all map configuration routes:
///     * one route to get a map's configuration;
///     * one route to create map configuration.
/// 
pub fn get_routes(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let configuration_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("configuration"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(create_map_configuration);

        configuration_creation_route
}

