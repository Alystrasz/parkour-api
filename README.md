# Parkour leaderboard API

### Routes

* `/v1/scores`:
    * **GET**: obtain the list of scores
    * **POST**: create a new score entry (with "name" and "time" keys in JSON body)

#### Development

```shell
# Run debug build
cargo run +key=KEY

# Build release build
cargo build -r
```