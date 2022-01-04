FROM rust:1.57-alpine as builder

WORKDIR /volume

RUN apk add --no-cache \
    build-base=~0.5 \
    mariadb-dev=~10.5 \
    musl-dev=~1.2 \
    postgresql-dev=~13.5 \
    sqlite-dev=~3.35

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release && \
    strip --strip-all target/release/sqlsherlock

FROM alpine:3.15

RUN addgroup -g 1000 sqlsherlock && \
    adduser -u 1000 -G sqlsherlock -D -g '' -H -h /dev/null -s /sbin/nologin marmalade

COPY --from=builder /volume/target/release/sqlsherlock /bin/

USER sqlsherlock

ENTRYPOINT ["/app/sqlsherlock"]
