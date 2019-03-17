FROM rust

RUN apt-get update
RUN apt-get install -y binutils nasm grub-pc-bin xorriso make
RUN rustup default nightly
RUN rustup component add rust-src
RUN cargo install xargo

WORKDIR /build
ENTRYPOINT make iso