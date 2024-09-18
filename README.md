# Parkour leaderboard API

### Global architecture

* `Events` are the basis entity on this API; think of them like Overwatch seasons, for instance. They have a beginning and an ending date (`start` and `end` fields, in *seconds* since Epoch), and link to several maps;
* `Maps` link in-game maps to events, and link to several routes;
* `MapRoutes` contain all information needed to setup a Parkour route (including in-game coordinates for map entities such as checkpoints and ziplines [a route example for the `mp_thaw` map is available in the `docs` directory]).
* `Scores` store players performances for each map;

All entities are stored in JSON files under the `data` directory.

### Routes

* `/v1/events`:
    * **GET**: obtain the list of events
    * **POST**: create a new event

* `/v1/events/:event_id/maps`
    * **GET**: obtain the list of maps associated to the event
    * **POST**: create a new map associated to the event

* `/v1/maps/:map_id/routes`
    * **GET**: get the map routes
    * **POST**: create a new route for the map

* `/v1/routes/:route_id/scores`
    * **GET**: obtain the list of scores associated to the route
    * **POST**: create a new score entry on the route scoreboard

A web scoreboard displaying current event scores is served on `/`.

#### Security

Before launching, you need to set up a secret which will be used to authenticate servers.

```shell
# On Windows:
$Env:PARKOUR_API_SECRET = "your_secret_here"
cargo run

# On Linux
PARKOUR_API_SECRET=your_secret_here cargo run
```

#### Development

```shell
# Run debug build
cargo run

# Build release build
cargo build -r

# Build release build without glibc
cargo build --target x86_64-unknown-linux-musl -r
```

#### Environment variables

* `PARKOUR_API_SAVE_TIMER`: duration (in minutes) between two state saves;
* `PARKOUR_API_SECRET`: token that must be provided by clients under the `authentication` header to access API resources
