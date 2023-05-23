FROM rust:latest

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
RUN cargo install cargo-watch

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN --mount=type=cache,target=/usr/local/cargo/registry cargo build
RUN rm -rf ./src

CMD cargo watch -x run
