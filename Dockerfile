FROM rust:slim AS build

RUN cargo install cargo-make cargo-nextest

FROM rust:slim

LABEL maintainer="mariorgzlpz@correo.ugr.es" version="0.0.5"

WORKDIR /app/test

RUN adduser test \
    && chown -R test /app/test

USER test

COPY --from=build /usr/local/cargo/bin/cargo-make /usr/local/cargo/bin/cargo-nextest /usr/local/bin/

ENV CARGO_TARGET_DIR=/tmp/cache/

ENTRYPOINT [ "cargo", "make", "test" ]