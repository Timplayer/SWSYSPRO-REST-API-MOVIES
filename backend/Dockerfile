FROM rust:1.68 as builder

RUN USER=root cargo new --bin backend
WORKDIR /backend

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/backend*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=builder backend/target/release/backend .

CMD ["./backend"]
