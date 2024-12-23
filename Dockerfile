FROM rust:slim AS build

RUN cargo install cargo-make cargo-nextest

COPY . .

RUN cargo make clean \
    && cargo make test

FROM debian:bookworm-slim

LABEL maintainer="mariorgzlpz@correo.ugr.es" version="0.0.5"

WORKDIR /app/test

RUN adduser test \
    && chown -R test:test /app/

USER test

COPY --from=build /target/debug/deps/arbitrage_bets* /app/

RUN find /app/ -type f -executable -name 'arbitrage_bets-*' ! -name '*.d' -exec mv {} /app/arbitrage_bets \;

ENV CARGO_TARGET_DIR=/tmp/cache/

ENTRYPOINT [ "/app/arbitrage_bets" ]