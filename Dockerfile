FROM rust:1.77.1

WORKDIR /app

COPY . .

RUN cargo install diesel_cli --no-default-features --features postgres \
    && cargo install cargo-watch

CMD ["cargo", "watch", "--why", "-x", "build"]