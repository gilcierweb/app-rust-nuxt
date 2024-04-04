###### Builder ######
FROM rust:latest-alpine AS builder

LABEL maintainer="gilcierweb@gmail.com"

RUN apk add build-base bash

ENV APP_HOME /app

WORKDIR $APP_HOME

# Copy our manifests
COPY ./api-rust/Cargo.lock ./Cargo.lock
COPY ./api-rust/Cargo.toml ./Cargo.toml

COPY ./api-rust $APP_HOME

# Build your program for release
RUN cargo build --release
RUN rm src/*.rs

RUN strip ./target/release/api-rust

###### Release binary rust ######

FROM rust:latest-alpine AS release

WORKDIR $APP_HOME

COPY --from=builder /app/target/release/api-rust .

EXPOSE 8000

CMD ["./api-rust"]

# Run the binary
#CMD ["./target/release/api-rust"]
