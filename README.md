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