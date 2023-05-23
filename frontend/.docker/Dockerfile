# Install nightly build of rust since leptos depends on it
FROM rustlang/rust:nightly AS chef

ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
# Needed for some weird errors on device-link error
ENV RUSTUP_PERMIT_COPY_RENAME=1

WORKDIR /app

RUN cargo install cargo-chef trunk
RUN curl -sLO https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64
RUN chmod +x tailwindcss-linux-x64
RUN mv tailwindcss-linux-x64 /usr/local/bin/tailwindcss

FROM chef AS planner
COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json

RUN cargo chef cook --recipe-path recipe.json

COPY . .
RUN cargo build

CMD trunk serve
