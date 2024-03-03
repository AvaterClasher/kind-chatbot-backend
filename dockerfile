FROM rust:1 as builder
WORKDIR /app
COPY . .
RUN cargo install --path .
FROM debian:bullseye-slim as runner
COPY --from=builder /usr/local/cargo/bin/kind-chatbot /usr/local/bin/kind-chatbot
RUN apt-get update && apt install -y openssl libssl-dev
ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000
CMD ["kind-chatbot"]