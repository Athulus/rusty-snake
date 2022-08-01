FROM rust:latest

EXPOSE 8080
COPY ./ ./
RUN cargo build --release
CMD ./target/release/rusty-snake