use std::{sync::Arc, time::SystemTime, fs::File, io::Read};

use chrono::{NaiveDateTime, DateTime, Utc};
use handlebars::{Handlebars, handlebars_helper};
use serde::{Serialize, Deserialize};
use serde_json::json;
use warp::{Filter, Reply, Rejection};

use crate::{Store, event::Event, log, scores::ScoreEntry};

const TEMPLATE_FILE: &str = "scoreboard/template.html";

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct RouteResult {
    id: String,
    name: String,
    map_name: String,
    scores: Vec<ScoreEntry>
}


fn render(hbs: Arc<Handlebars<'_>>, store: Store) -> impl warp::Reply
{
    // Find current event
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let events = store.clone().events_list.read().clone().into_iter();
    let corresponding_events: Vec<Event> = events.filter(|e| now >= e.start.try_into().unwrap() && now <= e.end.try_into().unwrap()).collect();
    if corresponding_events.len() != 1 {
        log::error(&format!("Expected one corresponding event, found {}.", corresponding_events.len()));
        std::process::exit(42);
    }

    // Find associated maps
    let event = corresponding_events.first().unwrap().clone();
    let event_id = event.id.unwrap();
    let maps = store.clone().maps_list.read().clone();
    if !maps.contains_key(&event_id) {
        log::error("Current event features no map.");
        std::process::exit(1);
    }
    let corresponding_maps = maps.get(&event_id).unwrap().clone();
    let maps_clone = corresponding_maps.clone();

    // Build route objects
    let mut results: Vec<RouteResult> = Vec::new();
    for map in maps_clone {
        let routes = store.clone().routes_list.read().clone();
        let map_id = map.id.unwrap();
        if !routes.contains_key(&map_id) {
            log::warn(&format!("No route was found for map {}, skipping.", &map_id));
            continue;
        }

        let corresponding_routes = routes.get(&map_id).unwrap().clone();
        let mut map_routes: Vec<RouteResult> = corresponding_routes.into_iter().map(|route| {
            return RouteResult {
                id: route.id.unwrap(),
                name: route.name,
                map_name: map.map_name.clone(),
                scores: Vec::new()
            }
        }).collect();
        results.append(&mut map_routes);
    }

    // Load up scores in `results`
    let scores = store.clone().scores_list.read().clone();
    for result in &mut results {
        let route_id = &result.id;
        if !scores.contains_key(route_id) {
            log::warn(&format!("No scores were found for route {}, skipping.", &route_id));
            continue;
        }
        result.scores = scores.get(route_id).unwrap().clone();
    }

    let template = WithTemplate {
        name: "template.html",
        value: json!({
            "event": corresponding_events.clone().first(),
            "maps": corresponding_maps,
            "results": results
        }),
    };

    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    
    warp::reply::html(render)
}


pub fn get_routes(store: Store) -> impl Filter<Extract = (impl Reply,), Error = Rejection> + Clone {
    // Load HTML template
    let mut file = match File::open(TEMPLATE_FILE) {
        Ok(file) => file,
        Err(_) => {
            log::info(&format!("\"{}\" template file was not found.", TEMPLATE_FILE));
            std::process::exit(3);
        }
    };
    let mut data = String::new();
    match file.read_to_string(&mut data) {
        Ok(_) => (),
        Err(err) => {
            log::error(&format!("Failed reading \"{}\" file [{}].", TEMPLATE_FILE, err));
            std::process::exit(2);
        }
    };

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", data)
        .unwrap();

    // Add a helper to have indexes starting from 1
    handlebars_helper!(score_index: |index: i64| index+1);
    hb.register_helper("score_index", Box::new(score_index));

    // Add a helper to reduce number of decimals
    handlebars_helper!(reddec: |time: f64| format!("{time:.3}"));
    hb.register_helper("reddec", Box::new(reddec));

    // Add a helper to format dates
    handlebars_helper!(date2: |timestamp: i64| {
        let naive = NaiveDateTime::from_timestamp_opt(timestamp, 0).unwrap();
        let datetime: DateTime<Utc> = DateTime::from_naive_utc_and_offset(naive, Utc);
        let newdate = datetime.format("%Y-%m-%d %H:%M:%S");
        newdate.to_string() + " UTC"
    });
    hb.register_helper("date2", Box::new(date2));

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move || render(hb.clone(), store.clone());

    // Static route to serve CSS and JS assets
    let static_assets = warp::path("assets").and(warp::fs::dir("scoreboard/assets"));

    let get_scoreboard_route = warp::get()
        .and(warp::path::end())
        .map(handlebars);

    static_assets.or(get_scoreboard_route)
}
