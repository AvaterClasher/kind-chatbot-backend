FROM rust:1.64.0-buster as builder
# install protobuf
RUN apt-get update
COPY Cargo.toml /usr/src/app/
WORKDIR /usr/src/app
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target x86_64-unknown-linux-musl --release --bin kind-chatbot
FROM gcr.io/distroless/static-debian11 as runner
# get binary
COPY --from=builder /usr/src/app/target/x86_64-unknown-linux-musl/release/kind-chatbot /
# set run env
EXPOSE 8000
# run it
CMD ["/kind-chatbot"]