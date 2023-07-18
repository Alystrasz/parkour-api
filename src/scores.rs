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

fn post_json() -> impl Filter<Extract = (ScoreEntry,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

async fn create_score_entry(
    map_id: String,
    entry: ScoreEntry,
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {

        // Check if provided map exists
        let scores_read_lock = store.scores_list.read();
        let optional_scores = scores_read_lock.get(&map_id);
        if optional_scores.is_none() {
            return Ok(warp::reply::with_status(
                warp::reply::json(&"{\"message\": \"Map not found.\"}"),
                StatusCode::NOT_FOUND,
            ));
        }

        let mut scores = optional_scores.unwrap().clone();
        let index = scores.iter().position(|e| e.name == entry.name).unwrap_or_else(|| { usize::MAX });
        if index != usize::MAX {
            let existing_entry = &scores[index];
            // If existing entry is better than new entry, we keep the new entry
            if entry.time >= existing_entry.time {
                return Ok(warp::reply::with_status(
                    warp::reply::json(&"{\"message\": \"Leaderboard contains a better score entry for this player.\"}"),
                    http::StatusCode::ALREADY_REPORTED,
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

        println!("1");

        // Restore list
        let mut write_lock = store.scores_list.write();
        write_lock.insert(map_id, scores.to_vec());

        println!("2");

        Ok(warp::reply::with_status(
            warp::reply::json(&"Score created."),
            http::StatusCode::CREATED,
        ))
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


    let score_creation_route = warp::post()
        .and(warp::path("v1"))
        .and(warp::path("maps"))
        .and(warp::path::param())
        .and(warp::path("scores"))
        .and(warp::path::end())
        .and(post_json())
        .and(store_filter.clone())
        .and_then(create_score_entry);

    return scores_list_route.or(score_creation_route);
}