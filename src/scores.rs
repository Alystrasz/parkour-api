use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use warp::{hyper::StatusCode, Filter, Reply, Rejection};

use crate::Store;

pub type ScoreEntries = HashMap<String, Vec<ScoreEntry>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScoreEntry {
    name: String,
    time: f32,
}

/// Retrives scores list associated to a map id.
/// 
async fn get_list(
    map_id: String,
    store: Store
    ) -> Result<impl Reply, Rejection> {

    let scores_read_lock = store.scores_list.read();
    if !scores_read_lock.contains_key(&map_id) {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"{\"message\": \"Map not found.\"}"),
            StatusCode::NOT_FOUND,
        ));
    }

    let scores = scores_read_lock.get(&map_id).unwrap();
    Ok(warp::reply::with_status(
        warp::reply::json(&scores),
        StatusCode::OK,
    ))
}

/// This middleware creates `Score` payloads from POST request bodies.
/// 
fn post_json() -> impl Filter<Extract = (ScoreEntry,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

/// Creates a score entry on a given map, based on its map identifier.
/// 
async fn create_score_entry(
    map_id: String,
    entry: ScoreEntry,
    store: Store
) -> Result<impl Reply, Rejection> {

    // Check if provided map exists
    let scores_map: ScoreEntries = store.scores_list.read().clone();
    let optional_scores = scores_map.get(&map_id);
    if optional_scores.is_none() {
        return Ok(warp::reply::with_status(
            warp::reply::json(&"Map not found."),
            StatusCode::NOT_FOUND,
        ))
    }

    let mut scores = optional_scores.unwrap().clone().to_vec();
    let index = scores.iter().position(|e| e.name == entry.name).unwrap_or(usize::MAX);
    if index != usize::MAX {
        let existing_entry = &scores[index];
        // If existing entry is better than new entry, we keep the new entry
        if entry.time >= existing_entry.time {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"message\": \"Leaderboard contains a better score entry for this player.\"}"),
                StatusCode::ALREADY_REPORTED,
            ));
        }
        // Else, we remove the existing entry
        else {
            scores.remove(index);
        }
    }

    // Create new entry
    scores.push(ScoreEntry { name: entry.name, time: entry.time });

    // Sort list by times
    scores.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());

    // Restore list
    let mut write_lock = store.scores_list.write();
    write_lock.insert(map_id, scores.to_vec());

    Ok(warp::reply::with_status(
        warp::reply::json(&"Score created."),
        StatusCode::CREATED,
    ))
}


/// Returns all score-associated routes:
///     * one route to list a map's scores;
///     * one route to create scores on a given map.
/// 
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


    let score_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter)
        .and_then(create_score_entry);

    scores_list_route.or(score_creation_route)
}
