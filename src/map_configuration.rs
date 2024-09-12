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
struct Robot {
    origin: [f64; 3],
    angles: [i64; 3],
    talkable_radius: i64,
    animation: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct StartIndicator {
    coordinates: [f64; 3],
    trigger_radius: i64
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct MapObject {
    coordinates: [f64; 3],
    angles: [f64; 3],
    scale: f64,
    model_name: String
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapConfiguration {
    start_line: Line,
    finish_line: Line,
    leaderboards: Leaderboards,
    checkpoints: Vec<[f64; 3]>,
    start: StartPosition,
    end: EndPosition,
    ziplines: Vec<[[f64; 3]; 2]>,
    robot: Robot,
    indicator: StartIndicator,
    entities: Option<Vec<MapObject>>
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
    mut entry: MapConfiguration,
    store: Store
) -> Result<impl Reply, Rejection> {

    let mut write_lock = store.configurations_list.write();

    // Init entities member if needed
    if entry.entities.is_none() {
        entry.entities = Some(Vec::new());
    }
    write_lock.insert(map_id, entry);

    Ok(warp::reply::with_status(
        warp::reply::json(&"Map configuration updated."),
        StatusCode::CREATED,
    ))
}


/// Get map configuration.
/// 
async fn get_map_configuration(
    map_id: String,
    store: Store
) -> Result<impl Reply, Rejection> {

    let configurations_read_lock = store.configurations_list.read();
    if !configurations_read_lock.contains_key(&map_id) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Configuration not found."),
            StatusCode::NOT_FOUND,
        ));
    }

    let configuration = configurations_read_lock.get(&map_id).unwrap();
    Ok(warp::reply::with_status(
        warp::reply::json(&configuration),
        StatusCode::OK,
    ))
}


/// Returns all map configuration routes:
///     * one route to get a map's configuration;
///     * one route to create map configuration.
/// 
pub fn get_routes(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let configuration_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("configuration"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(create_map_configuration);

    let get_configuration_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("configuration"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_map_configuration);

    configuration_creation_route.or(get_configuration_route)
}

