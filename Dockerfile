FROM docker.io/library/rust:1.64.0-slim-bullseye AS chef

RUN cargo install cargo-chef
WORKDIR /app/


FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json


FROM chef AS builder

ENV DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
  && apt-get install --no-install-suggests --no-install-recommends --yes \
  libssl-dev=1.1.1n-0+deb11u3 \
  pkg-config=0.29.2-1

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release --bin rflector


FROM docker.io/library/debian:bullseye-slim AS sysprep

RUN groupadd --system rflector && useradd --system --shell /bin/false --gid rflector rflector


FROM gcr.io/distroless/cc:latest AS runtime

COPY --from=sysprep /etc/passwd /etc/passwd
COPY --from=sysprep /etc/group /etc/group

COPY --from=builder --chown=rflector:rflector /app/target/release/rflector /usr/local/bin/

USER rflector

ENTRYPOINT ["/usr/local/bin/rflector"]
