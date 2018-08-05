FROM rust:latest

WORKDIR /usr/src/learning_rust

COPY . .

RUN cargo install

CMD ["RUST_LOG=main", "learning_rust"]