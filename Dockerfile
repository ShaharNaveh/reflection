FROM docker.io/library/debian:bullseye-slim AS sysprep

RUN groupadd --system rflector && useradd --system --shell /bin/false --gid rflector rflector

FROM docker.io/library/rust:1.64.0-slim-bullseye AS build-env

ENV DEBIAN_FRONTEND=noninteractive

RUN apt update \
  && apt install --no-install-suggests --no-install-recommends --yes \
  libssl-dev \
  pkg-config

WORKDIR /app/

RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs

COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

RUN rm --recursive ./src/ ./target/release/
COPY ./src ./src
RUN \
  touch -a -m ./src/main.rs \
  && cargo build --release

FROM gcr.io/distroless/cc:latest

COPY --from=sysprep /etc/passwd /etc/passwd
COPY --from=sysprep /etc/group /etc/group

COPY --from=build-env --chown=rflector:rflector /app/target/release/rflector /usr/local/bin/
USER rflector

ENTRYPOINT ["/usr/local/bin/rflector"]
