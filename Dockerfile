###### Builder ######
FROM rust:latest-alpine AS builder

LABEL maintainer="gilcierweb@gmail.com"

RUN apk add build-base bash musl-dev

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

#RUN cargo install --path . --target=x86_64-unknown-linux-musl
# remove the dummy build.
#RUN cargo clean -p $project_name_specified_in_cargo

###### Release binary rust ######

FROM rust:latest-alpine AS release

WORKDIR $APP_HOME

COPY --from=builder /app/target/release/api-rust .

# build with x86_64-unknown-linux-musl to make it runs on alpine.
#RUN cargo install --path . --target=x86_64-unknown-linux-musl
#COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin

EXPOSE 8000

CMD ["./api-rust"]

# Run the binary
#CMD ["./target/release/api-rust"]
