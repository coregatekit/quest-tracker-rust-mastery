FROM rust:1.89-bullseye AS builder

WORKDIR /usr/src/quests-tracker
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
  libssl-dev \
  libpq-dev \
  ca-certificates \
  && rm -rf /var/lib/api/lists/*

COPY --from=builder /usr/local/cargo/bin/quest-tracker-rust-mastery /usr/local/bin/quest-tracker-rust-mastery

CMD [ "quest-tracker-rust-mastery" ]
