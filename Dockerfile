FROM rust:slim

LABEL maintainer="mariorgzlpz@correo.ugr.es" version="0.0.5"

WORKDIR /app/test

RUN adduser --disabled-login --uid 1001 test && chown -R test:test /app/test

USER test

COPY Cargo.toml Cargo.lock Makefile.toml ./

COPY src/ ./src/ 

COPY data/ ./data/

RUN cargo install cargo-make cargo-nextest

RUN cargo build --release

CMD ["cargo", "make", "test"]