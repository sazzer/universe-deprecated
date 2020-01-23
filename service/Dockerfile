FROM debian:stretch-slim AS package-cache

WORKDIR /universe
COPY Cargo.lock Cargo.toml docker/package.sh /universe/
COPY ./crates /universe/crates/
RUN ./package.sh

# Build the code first
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /universe

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY --from=package-cache /universe /universe
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /universe/target -name "*universe*" | xargs rm -rf

# Finally, we do a full build of our code
RUN rm -rf crates
COPY crates /universe/crates/
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM debian:stretch-slim

WORKDIR /universe
COPY --from=builder /universe/target/release/universe /universe/universe
COPY migrations /universe/migrations/

CMD ["/universe/universe"]
