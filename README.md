# Parkour leaderboard API

### Routes

* `/v1/scores`:
    * **GET**: obtain the list of scores
    * **POST**: create a new score entry (with "name" and "time" keys in JSON body)

#### Security

Before launching, you need to setup a secret which will be used to authenticate servers.

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
