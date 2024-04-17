##### BUILDER #####
FROM rust:latest as builder

WORKDIR /

COPY . .

RUN cargo install --path .

##### RUNNER ######
FROM debian:bookworm

COPY --from=builder /usr/local/cargo/bin/node-tester /usr/local/bin/node-tester

RUN apt-get update

CMD node-tester