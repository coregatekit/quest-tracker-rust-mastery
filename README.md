# Quest tracker Rust Mastery

## Prerequisites

- rust v1.89.0+
- podman or any container runtime that you prefer
- database management tools such as DBeaver, DataGrip or any you prefer

## Get started

Create an environment file before start the project.

```env
STAGE=Local

SERVER_PORT=8080
SERVER_BODY_LIMIT=10 # MB
SERVER_TIMEOUT=90 # seconds

DATABASE_URL=postgres://postgres:123456@localhost/quests_tracker_db

JWT_ADVENTURER_SECRET=a_supersecret
JWT_ADVENTURER_REFRESH_SECRET=ar_supersecretrefresh
JWT_GUILD_COMMANDER_SECRET=g_supersecret
JWT_GUILD_COMMANDER_REFRESH_SECRET = gr_supersecretrefresh
```

And start a database instance with podmand or docker<br>
In this project we will `postgresql` as a database

```bash
podman run --name quests-tracker -e POSTGRES_PASSWORD=123456 -p 5432:5432 -d postgres:17
```

To start the project use this command

```bash
cargo run
```

Or use this command to start the project hot reload but you have to install `cargo-watch` before use this command

```bash
# install cargo-watch
cargo install cargo-watch
```

```bash
cargo watch -x run
```

## Unit testing

To run unit test use this command

```bash
cargo test
```

Install package `cargo-tarpaulin` for testing with code coverage

```bash
cargo install cargo-tarpaulin
```

and run unit test with coverage with this command

```bash
cargo tarpaulin --out xml
```

## API Document

You can import postman collection to test the api `Quests tracker.postman_collection.json`