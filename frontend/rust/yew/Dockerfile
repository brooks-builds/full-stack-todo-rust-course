FROM rust:1.58
VOLUME /code
RUN rustup target add wasm32-unknown-unknown
RUN cargo install trunk
EXPOSE 8080
WORKDIR /code
CMD trunk serve