FROM rustlang/rust:nightly
WORKDIR /app
RUN apt-get update -y && apt-get upgrade -y
RUN cargo install cargo-watch

COPY ./server/Cargo.toml ./Cargo.toml
COPY ./server/Cargo.lock ./Cargo.lock
COPY ./server/src ./src

EXPOSE 8080
#RUN cargo build --release
#CMD ["./target/release/server"]
CMD ["cargo", "watch", "-x", "run"]