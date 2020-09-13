FROM rust:1.45-alpine

WORKDIR /usr/src/crikey-irc

COPY Cargo.lock Cargo.lock
COPY Cargo.toml Cargo.toml

COPY src src
COPY tests tests

CMD cargo run
