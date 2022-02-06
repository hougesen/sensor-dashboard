FROM rust:1.56-slim-buster AS base
ENV USER=root
ENV SERVER_ADDR=0.0.0.0:5000
ENV PG.USER=postgres
ENV PG.PASSWORD=example
ENV PG.HOST=127.0.0.1
ENV PG.PORT=5432
ENV PG.DBNAME=sensor
ENV PG.POOL.MAX_SIZE=16
WORKDIR /code
RUN cargo init
COPY Cargo.toml /code/Cargo.toml
RUN cargo fetch
COPY src /code/src
CMD [ "cargo", "test", "--offline" ]

FROM base AS builder
ENV SERVER_ADDR=0.0.0.0:5000
ENV PG.USER=postgres
ENV PG.PASSWORD=example
ENV PG.HOST=127.0.0.1
ENV PG.PORT=5432
ENV PG.DBNAME=sensor
ENV PG.POOL.MAX_SIZE=16
RUN cargo build --release --offline

FROM debian:buster-slim
ENV SERVER_ADDR=0.0.0.0:5000
ENV PG.USER=postgres
ENV PG.PASSWORD=example
ENV PG.HOST=127.0.0.1
ENV PG.PORT=5432
ENV PG.DBNAME=sensor
ENV PG.POOL.MAX_SIZE=16
COPY --from=builder /code/target/release/api /usr/bin/api

EXPOSE 5000

ENTRYPOINT [ "/usr/bin/api" ]