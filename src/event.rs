use serde::{Serialize, Deserialize};

use crate::Store;

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Event {
    name: String,
    id: String,
}

pub async fn get_list(
    store: Store
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let r = store.events_list.read();
        Ok(warp::reply::json(&*r))
}