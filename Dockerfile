FROM debian:13.0-slim AS chef
ARG RUST_TARGET
WORKDIR /app

RUN apt-get update \
    && apt-get install --no-install-recommends -y libqpdf-dev gcc-aarch64-linux-gnu \
    pkg-config build-essential curl git libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN rustup target add ${RUST_TARGET}
RUN cargo install cargo-chef --locked

FROM chef AS backend_planner

Copy migrations ./migrations/
COPY .sqlx ./.sqlx/
COPY crates ./crates/
COPY Cargo.toml ./
COPY Cargo.lock ./

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS backend_builder

COPY --from=backend_planner /app/recipe.json ./

RUN cargo chef cook --release --target ${RUST_TARGET} --recipe-path recipe.json

Copy migrations ./migrations/
COPY .sqlx ./.sqlx/
COPY crates ./crates/
COPY Cargo.toml ./
COPY Cargo.lock ./

RUN cargo build --release --target ${RUST_TARGET} --package spoolman-api

FROM debian:13.0-slim AS runtime
ARG RUST_TARGET
WORKDIR /app

RUN apt-get update \
    && apt-get install --no-install-recommends -y libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=backend_builder /app/target/${RUST_TARGET}/release/spoolman-api ./

ENTRYPOINT ["./spoolman-api"]