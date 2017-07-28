FROM debian:jessie

ENV RUST_VERSION=1.19.0

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    libgl1-mesa-dev \
    wget \
    git subversion \
    nano

RUN wget https://static.rust-lang.org/dist/rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz

RUN tar -xzf rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz && \
    rust-$RUST_VERSION-x86_64-unknown-linux-gnu/install.sh --without=rust-docs && \
    rm -rf \
        rust-$RUST_VERSION-x86_64-unknown-linux-gnu \
        rust-$RUST_VERSION-x86_64-unknown-linux-gnu.tar.gz

WORKDIR /src

CMD cargo run
