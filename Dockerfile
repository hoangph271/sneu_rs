FROM rust:latest

WORKDIR /usr/src/sneu_rs
COPY . .

RUN cargo install --path .

CMD ["sneu_rs"]
