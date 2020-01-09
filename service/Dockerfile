# Build the code first
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /universe

# We do a non-code build to get all the dependencies. This allows us to benefit from layer caching
COPY Cargo.lock Cargo.toml /universe/
RUN mkdir /universe/src
RUN touch /universe/src/lib.rs
RUN echo 'fn main() {println!("Wrong");}' > /universe/src/main.rs
RUN cargo build --release

# Then we trash our actual build so that we can correctly build again
RUN find /universe/target -name *universe*
RUN find /universe/target -name *universe* | xargs rm -rf

# Finally, we do a full build of our code
COPY src /universe/src/
RUN cargo build --release


# Next build a container with the build artifact but no code
FROM debian:stretch-slim

WORKDIR /universe
COPY --from=builder /universe/target/release/universe /universe/universe
COPY messages /universe/messages/
COPY migrations /universe/migrations/
COPY static /universe/static/
COPY templates /universe/templates/

CMD ["/universe/universe"]
