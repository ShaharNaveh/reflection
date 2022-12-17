ARG BASE_IMAGE="docker.io/library/rust:1.66.0-slim-bullseye"

FROM $BASE_IMAGE as chef
RUN cargo install cargo-chef --version 0.1.50
WORKDIR /app/

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
RUN apt-get update \
  && apt-get install --no-install-suggests --no-install-recommends --yes \
  libssl-dev=1.1.1n-0+deb11u3 \
  pkg-config=0.29.2-1

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin reflection

FROM gcr.io/distroless/cc AS runtime
COPY --from=builder /app/target/release/reflection /usr/local/bin/
ENTRYPOINT ["/usr/local/bin/reflection"]
