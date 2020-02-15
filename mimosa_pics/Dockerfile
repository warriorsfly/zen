# build stage
FROM rust:latest as build

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# create new empty binary project
RUN USER=root cargo new --bin mimosa_pics
WORKDIR /mimosa_pics

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
RUN rm ./target/release/deps/mimosa_pics*
RUN cargo build --release
# deploy stage
FROM debian:stretch-slim

# create app directory
RUN mkdir mimosa_pics
WORKDIR /mimosa_pics

# install libpq
RUN apt-get update
RUN apt-get install -y libpq-dev
RUN rm -rf /var/lib/apt/lists/*

# copy binary and configuration files
COPY --from=build /mimosa_pics/target/release/mimosa_pics .
COPY --from=build /mimosa_pics/.env .
COPY --from=build /mimosa_pics/diesel.toml .

# expose port
EXPOSE 8000

# run the binary
ENTRYPOINT ["/mimosa_pics/mimosa_pics"]