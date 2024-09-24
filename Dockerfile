# 1. This tells docker to use the Rust official image
FROM rust:latest

# 2. Copy the files in your machine to the Docker image
COPY . .

RUN apt-get update && apt install libgtk-4-dev -y && apt-get install librust-atk-dev -y && apt-get install libgtk-3-dev

# Build your program for release
RUN cargo run

# Run the binary
CMD ["./target/release/dressMe"]

ENV SYSTEM_DEPS_PANGO_NO_PKG_CONFIG="true"
ENV SYSTEM_DEPS_PANGO_LIB="pango-1.0"