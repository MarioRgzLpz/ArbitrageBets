FROM rust:slim

LABEL maintainer="mariorgzlpz@correo.ugr.es" version="0.0.5"

WORKDIR /app/test

COPY Cargo.toml Makefile.toml /app/

RUN adduser test \
    && chown -R test:test /app/

USER test

RUN mkdir -p /app/src \
	&& touch /app/src/lib.rs \
	&& cargo update \
	&& rm -rf /app/src \
	&& ln -s /app/test/src /app/src 

RUN cargo install cargo-make cargo-nextest

ENV CARGO_TARGET_DIR=/tmp/cache/

ENTRYPOINT [ "cargo", "make", "test" ]