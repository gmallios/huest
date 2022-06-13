FROM rust:latest
WORKDIR /workspace/huest
RUN apt-get update && apt-get install -y \
    libclang-dev \
    avahi-daemon \
    libavahi-client-dev \
 && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml .
COPY Cargo.lock .
RUN mkdir src && touch src/main.rs
RUN echo "fn main() { println!(\"Hello, world!\"); }" > src/main.rs
RUN cargo build --release
RUN rm src/main.rs
