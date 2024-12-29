use std::collections::HashMap;
use uuid::Uuid;
use warp::{hyper::StatusCode, Filter, Reply, Rejection};

use crate::Store;
use serde::{Serialize, Deserialize};


pub type MapRoutes = HashMap<String, Vec<MapRoute>>;


#[derive(Debug, Deserialize, Serialize, Clone)]
struct Line {
    origin: [f64; 3],
    angles: [i64; 3],
    dimensions: [i64; 2],
    trigger: [[f64; 3]; 2]
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RouteName {
    origin: [f64; 3],
    angles: [i64; 3],
    dimensions: [i64; 2],
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
    model_name: String,
    hidden: Option<bool>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MapRoute {
    pub id: Option<String>,
    pub name: String,
    start_line: Line,
    finish_line: Line,
    leaderboards: Leaderboards,
    checkpoints: Vec<[f64; 3]>,
    start: StartPosition,
    end: EndPosition,
    ziplines: Vec<[[f64; 3]; 2]>,
    perks: Option<HashMap<String, String>>,
    robot: Robot,
    indicator: StartIndicator,
    route_name: RouteName,
    entities: Option<Vec<MapObject>>
}


/// This middleware creates `MapRoute` payloads from POST request bodies.
/// 
pub fn post_json() -> impl Filter<Extract = (MapRoute,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}


/// Creates a map route, based on its map identifier.
/// 
async fn create_map_route(
    map_id: String,
    mut entry: MapRoute,
    store: Store
) -> Result<impl Reply, Rejection> {

    // Check if provided map exists
    let routes_list = store.routes_list.read().clone();
    let map_routes = routes_list.get(&map_id);
    if map_routes.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Map not found."),
            StatusCode::NOT_FOUND,
        ))
    }

    let mut routes = map_routes.unwrap().clone();
    let index = routes.iter().position(|route| route.name == entry.name).unwrap_or(usize::MAX);
        if index != usize::MAX {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"error\": \"Route name already used.\"}"),
                StatusCode::ALREADY_REPORTED,
            ));
        }

    // Insert new route
    let route_id = Uuid::new_v4().to_string();
    entry.id = Some(route_id.clone());
    if entry.perks.is_none() {
        entry.perks = Some(HashMap::new());
    }
    if entry.entities.is_none() {
        entry.entities = Some(Vec::new());
    }
    routes.push(entry);
    let mut write_lock = store.routes_list.write();
    write_lock.insert(map_id, routes);

    // Create associated scores
    let mut scores_write_lock = store.scores_list.write();
    scores_write_lock.insert(route_id, [].to_vec());

    Ok(warp::reply::with_status(
        warp::reply::json(&"Map route created."),
        StatusCode::CREATED,
    ))
}


/// Get map routes.
/// 
async fn get_map_routes(
    map_id: String,
    store: Store
) -> Result<impl Reply, Rejection> {

    let routes_read_lock = store.routes_list.read();
    if !routes_read_lock.contains_key(&map_id) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Route not found."),
            StatusCode::NOT_FOUND,
        ));
    }

    let routes = routes_read_lock.get(&map_id).unwrap();
    Ok(warp::reply::with_status(
        warp::reply::json(&routes),
        StatusCode::OK,
    ))
}


/// Returns all map routing routes:
///     * one route to get a map's routes;
///     * one route to create map routes.
/// 
pub fn get_routes(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let route_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("routes"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(create_map_route);

    let get_routes_route = warp::get()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("routes"))
        .and(warp::path::end())
        .and(store_filter)
        .and_then(get_map_routes);

    route_creation_route.or(get_routes_route)
}

