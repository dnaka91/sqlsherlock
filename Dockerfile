FROM rust:1.61-alpine as builder

WORKDIR /volume

RUN apk add --no-cache \
    build-base=~0.5 \
    mariadb-dev=~10.6 \
    musl-dev=~1.2 \
    postgresql14-dev=~14.3 \
    sqlite-dev=~3.36

COPY src/ src/
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release

FROM alpine:3.16

RUN addgroup -g 1000 sqlsherlock && \
    adduser -u 1000 -G sqlsherlock -D -g '' -H -h /dev/null -s /sbin/nologin marmalade

COPY --from=builder /volume/target/release/sqlsherlock /bin/

USER sqlsherlock

ENTRYPOINT ["/app/sqlsherlock"]
