use std::{sync::Arc, time::SystemTime, fs::File, io::Read};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::{Filter, Reply, Rejection};

use crate::{Store, event::Event, log};

const TEMPLATE_FILE: &str = "scoreboard/template.html";

struct WithTemplate<T: Serialize> {
    name: &'static str,
    value: T,
}


fn render<T>(template: WithTemplate<T>, hbs: Arc<Handlebars<'_>>) -> impl warp::Reply
where
    T: Serialize,
{
    let render = hbs
        .render(template.name, &template.value)
        .unwrap_or_else(|err| err.to_string());
    warp::reply::html(render)
}

pub fn get_routes(store: Store) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
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

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    // Static route to serve CSS and JS assets
    let static_assets = warp::path("assets").and(warp::fs::dir("scoreboard/assets"));

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

    let get_scoreboard_route = warp::get()
        .and(warp::path::end())
        .map(move || WithTemplate {
            name: "template.html",
            value: json!({
                "event": corresponding_events.clone().first(),
                "maps": corresponding_maps,
                "scores": store.clone().scores_list.read().get("b75bd077-7198-4c5a-ba32-33e16202f320").unwrap().to_vec()
            }),
        })
        .map(handlebars);

    static_assets.or(get_scoreboard_route)
}
