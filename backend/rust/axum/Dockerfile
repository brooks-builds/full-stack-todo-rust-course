FROM rust:latest

RUN cargo install --locked cargo-watch
RUN mkdir /code

COPY project_solution /code

WORKDIR /code

EXPOSE 3000

CMD ["cargo", "watch", "-x", "run"]
