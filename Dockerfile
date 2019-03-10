FROM rust

RUN apt-get update
RUN apt-get install -y binutils nasm grub-common xorriso make
RUN rustup default nightly
RUN rustup component add rust-src
RUN cargo install xargo
