###### Builder ######
FROM rust:1.77-buster as builder

LABEL maintainer="gilcierweb@gmail.com"

RUN apt update
RUN apt install -y libpq-dev bash

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY ./Cargo.toml .
COPY ./diesel.toml .
COPY . .

# Build application
RUN cargo install --path .
RUN cargo build --release --bin seed

ENTRYPOINT ["diesel"]

# Build for release
FROM debian:buster-slim AS release

RUN apt update
RUN apt install -y libpq-dev

WORKDIR /user/local/bin

COPY --from=builder /app/target/release/api-rust .
COPY --from=builder /app/target/release/seed .
COPY --from=builder /usr/local/cargo/bin/diesel .
COPY --from=builder /app/migrations ./migrations

EXPOSE 8080

CMD [ "./api-rust", "./seed", "./diesel" ]