# Parkour leaderboard API

### Routes

* `/v1/scores`:
    * **GET**: obtain the list of scores
    * **POST**: create a new score entry (with "name" and "time" keys in JSON body)

#### Development

Before launching, you need to create a `.env.key` file containing an authentication token.

```shell
# Run debug build
cargo run

# Build release build
cargo build -r

# Build release build without glibc
cargo build --target x86_64-unknown-linux-musl -r
```