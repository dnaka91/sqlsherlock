# syntax = docker/dockerfile:experimental
FROM clux/muslrust:stable as builder

RUN apt-get update \
&& apt-get install -y libmysqlclient-dev \
&& rm -rf /var/lib/apt/lists/*

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN --mount=type=cache,target=/root/.cargo/git \
    --mount=type=cache,target=/root/.cargo/registry \
    --mount=type=cache,target=/volume/target \
    cargo install --path .

FROM alpine:3.11

COPY --from=builder /root/.cargo/bin/sqlsherlock /app/

ENTRYPOINT ["/app/sqlsherlock"]
