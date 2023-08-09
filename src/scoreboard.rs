use std::{sync::Arc, time::SystemTime};

use handlebars::Handlebars;
use serde::Serialize;
use serde_json::json;
use warp::{Filter, Reply, Rejection};

use crate::{Store, event::Event, log};

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
    let template = "<!DOCTYPE html>
        <html>
            <head>
                <title>Parkour scoreboard</title>
                <link href=\"assets/style.css\" rel=\"stylesheet\" />
            </head>
            <body>
                <h1>Parkour scoreboard</h1>
                <nav>
                    <h1>{{event.name}}</h1>
                    <h2>{{event.description}}</h2>
                </nav>
                <table>
                    <tr id=\"header\">
                        <th>Position</th>
                        <th>Player name</th>
                        <th>Time (seconds)</th>
                    </tr>
                    {{#each scores}}
                    <tr>
                        <td>{{@index}}</td>
                        <td>{{this.name}}</td>
                        <td>{{this.time}}</td>
                    </tr>
                    {{/each}}
                </table>
            </body>
        </html>";

    let mut hb = Handlebars::new();
    // register the template
    hb.register_template_string("template.html", template)
        .unwrap();

    // Turn Handlebars instance into a Filter so we can combine it
    // easily with others...
    let hb = Arc::new(hb);

    // Create a reusable closure to render template
    let handlebars = move |with_template| render(with_template, hb.clone());

    // Static route to serve CSS and JS assets
    let static_assets = warp::path("assets").and(warp::fs::dir("assets"));

    // Find current event
    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs();
    let events = store.clone().events_list.read().clone().into_iter();
    let corresponding_events: Vec<Event> = events.filter(|e| now >= e.start.try_into().unwrap() && now <= e.end.try_into().unwrap()).collect();
    if corresponding_events.len() != 1 {
        log::error(&format!("Expected one corresponding event, found {}.", corresponding_events.len()));
        std::process::exit(42);
    }

    let get_scoreboard_route = warp::get()
        .and(warp::path::end())
        .map(move || WithTemplate {
            name: "template.html",
            value: json!({
                "event": corresponding_events.clone().first(),
                "scores": store.clone().scores_list.read().get("b75bd077-7198-4c5a-ba32-33e16202f320").unwrap().to_vec()
            }),
        })
        .map(handlebars);

    static_assets.or(get_scoreboard_route)
}
