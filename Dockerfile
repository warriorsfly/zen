ARG base=rust:latest
# Our first FROM statement declares the build environment.
FROM ${base} AS builder
# Build our application.
ADD . ./
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `mimosa`.
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/mimosa \
    /usr/local/bin/
CMD /usr/local/bin/mimosa