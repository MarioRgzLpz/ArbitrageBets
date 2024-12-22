FROM rust:slim AS build

RUN cargo install cargo-make cargo-nextest

FROM rust:slim

LABEL maintainer="mariorgzlpz@correo.ugr.es" version="0.0.5"

WORKDIR /app/test

RUN adduser test \
    && chown -R test:test /app/

USER test

RUN ln -s /app/test/src /app/src

COPY --from=build /usr/local/cargo/bin/cargo-make /usr/local/bin/cargo-make
COPY --from=build /usr/local/cargo/bin/cargo-nextest /usr/local/bin/cargo-nextest

ENV CARGO_TARGET_DIR=/tmp/cache/

ENTRYPOINT [ "cargo", "make", "test" ]