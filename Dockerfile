FROM rust:latest

RUN dpkg --add-architecture armhf \
    apt-get update && \
    apt-get install -y \
    libclang-dev \
    avahi-daemon \
    libavahi-client-dev:armhf \
    gcc-arm-linux-gnueabihf \
    binutils-arm-linux-gnueabihf \
 #Not needed i think && apt-get --reinstall install libc6 libc6-dev \   
 && rm -rf /var/lib/apt/lists/*
# RUN cp -rf /usr/arm-linux-gnueabihf/include/*  /usr/include
RUN rustup target add arm-unknown-linux-gnueabihf
RUN bash -c 'echo -e  dpkg --print-architecture'
WORKDIR /workspace/huest
COPY .cargo ./.cargo
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# RUN cargo build --release --target arm-unknown-linux-gnueabihf
SHELL ["/bin/sh", "-c"]