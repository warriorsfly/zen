# build stage
FROM rust:latest as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin mimosa_account
WORKDIR /mimosa_account

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
RUN rm ./target/release/deps/mimosa_account*
RUN cargo build --release
# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir mimosa_account
WORKDIR /mimosa_account

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /mimosa_account/target/release/mimosa_account .
COPY --from=build /mimosa_account/.env .
COPY --from=build /mimosa_account/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/mimosa_account/mimosa_account"]