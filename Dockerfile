FROM rust:latest

WORKDIR /usr/src/learning_rust

COPY . .

RUN cargo install

CMD ["learning_rust"]