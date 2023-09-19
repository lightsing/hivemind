FROM rust:1.72-bookworm as chef
RUN cargo install cargo-chef cargo-zigbuild
RUN apt update && apt install -y python3-pip protobuf-compiler libssl-dev && rm -rf /var/lib/apt/lists/*
RUN pip3 install --break-system-packages ziglang
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --zigbuild --workspace --release --recipe-path recipe.json
COPY . .
RUN cargo zigbuild --release

FROM debian:bookworm-slim AS runtime-base
RUN apt update && apt install -y libssl3 && rm -rf /var/lib/apt/lists/*

FROM runtime-base AS hive-bee
COPY --from=builder /app/target/release/hive-bee /usr/local/bin/hive-bee
ENTRYPOINT ["/usr/local/bin/hive-bee"]

FROM runtime-base AS hivectl
COPY --from=builder /app/target/release/hivectl /usr/local/bin/hivectl
ENTRYPOINT ["/usr/local/bin/hivectl"]

FROM runtime-base AS hive-queen
COPY --from=builder /app/target/release/hive-queen /usr/local/bin/hive-queen
ENTRYPOINT ["/usr/local/bin/hive-queen"]