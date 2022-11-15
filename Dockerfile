FROM rust:1.62.1-slim as builder
RUN apt-get update \
    && apt-get install -y libssl-dev  \
    && rm -rf /var/lib/apt/lists/* 

FROM builder as build
COPY . /opt/dotfiles-rs/
WORKDIR /opt/dotfiles-rs
RUN cargo install --path .

FROM debian:bookworm-slim as exec
RUN apt-get update \
    && apt-get install -y libssl-dev  \
    && rm -rf /var/lib/apt/lists/* 
COPY --from=build /usr/local/cargo/bin/dotfiles-rs /usr/local/bin/dotfiles-rs
RUN /usr/local/bin/dotfiles-rs -h
ENTRYPOINT ["/usr/local/bin/dotfiles-rs"]
