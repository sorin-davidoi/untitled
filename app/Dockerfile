FROM rust:1.53.0-slim AS base

RUN apt-get update && apt-get --no-install-recommends --assume-yes install pkg-config libssl-dev curl && apt-get clean

WORKDIR /app

RUN useradd rust

RUN chown -R rust:rust /app

USER rust:rust

FROM base as development

USER root:root

RUN apt-get update && apt-get --no-install-recommends --assume-yes install lld && apt-get clean

USER rust:rust

RUN rustup component add rustfmt
RUN rustup component add clippy
RUN cargo install cargo-watch --version 7.8.0

# Create the directories with the right permissions
# (otherwise they will be owned by the host user if created
# automatically by Docker when the volumes are mounted)
RUN mkdir /usr/local/cargo/git && mkdir /app/target

ENV CARGO_BUILD_RUSTFLAGS="-C link-arg=-fuse-ld=lld"

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_IDENT=false
ENV ROCKET_SHUTDOWN={grace=0,mercy=0}

CMD [ "cargo", "watch", "--exec", "run $CARGO_ARGS" ]
