FROM rust:latest

RUN cargo install cargo-watch
RUN apt update

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build
RUN rm -rf ./src

CMD cargo watch -x run
