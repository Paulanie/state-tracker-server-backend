FROM rust:bookworm as build

RUN apt update
RUN apt install -y libssl-dev

RUN USER=root cargo new --bin state-tracker-backend
WORKDIR /state-tracker-backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt update
RUN apt install -y libssl-dev

# COPY --from=build /state-tracker-backend/target/release/state-tracker-backend/target/x86_64-unknown-linux-musl/release/state-tracker-backend .

WORKDIR /usr/src/
COPY --from=build /state-tracker-backend/target/release/state-tracker-backend ./state-tracker-backend
COPY ./application-docker.toml ./application.toml

ENV RUST_LOG="info,rbatis=warn"
# Run the binary
CMD ["./state-tracker-backend"]