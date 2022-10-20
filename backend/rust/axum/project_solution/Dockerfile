FROM rust:latest
RUN cargo install cargo-watch
ENV PORT=3000
EXPOSE 3000
VOLUME [ "/code" ]
WORKDIR /code
CMD cargo watch -x run
