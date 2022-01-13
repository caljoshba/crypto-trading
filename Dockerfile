FROM rust

WORKDIR /code

COPY Cargo.toml ./
COPY Cargo.lock ./
COPY src ./src
COPY config ./config

RUN cargo build

CMD ["cargo", "run"]