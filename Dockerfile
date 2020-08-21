FROM rust:1.45-alpine

WORKDIR /usr/src/irustc-bot

COPY . .

CMD cargo run
