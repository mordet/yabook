# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:latest as cargo-build
RUN apt-get update
RUN apt-get install musl-tools -y
RUN rustup install nightly && rustup default nightly
WORKDIR /usr/src/backend
COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/backend*
COPY . .
RUN cargo build --release
# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM ubuntu:latest
RUN addgroup --gid 1000 backend
RUN adduser --shell /bin/sh --uid 1000 --ingroup backend backend
WORKDIR /home/backend/bin/
COPY --from=cargo-build /usr/src/backend/target/release/backend .
RUN chown backend:backend backend
USER backend
EXPOSE 8080
CMD ["./backend"]
