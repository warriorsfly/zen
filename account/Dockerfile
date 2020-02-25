# build stage
FROM rust:latest as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin account
WORKDIR /account

# copy manifests
COPY ./Cargo.toml ./Cargo.toml

# build this project to cache dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy project source and necessary files
COPY ./src ./src
COPY ./migrations ./migrations
COPY ./diesel.toml .

# add .env and secret.key for Docker env
RUN touch .env
# RUN mv src/secret.key.sample src/secret.key

# rebuild app with project source
RUN rm ./target/release/deps/account*
RUN cargo build --release
# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir account
WORKDIR /account

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /account/target/release/account .
COPY --from=build /account/.env .
COPY --from=build /account/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/account/account"]