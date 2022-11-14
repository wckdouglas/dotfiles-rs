FROM rust:1.62.1 as builder
RUN apt-get update && rm -rf /var/lib/apt/lists/*

FROM builder as build
COPY . /opt/dotfiles-rs/
WORKDIR /opt/dotfiles-rs
RUN cargo install --path .

FROM debian:bookworm-slim as exec
RUN apt-get update &&  \
    rm -rf /var/lib/apt/lists/* && \
    apt install openssl
COPY --from=build /usr/local/cargo/bin/dotfiles-rs /usr/local/bin/dotfiles-rs
ENTRYPOINT ["/usr/local/bin/dotfiles-rs"]
