FROM rust:1 AS builder

# Installing wasm target
RUN rustup target add wasm32-unknown-unknown

# Installing dioxus-cli
RUN cargo install dioxus-cli

# Setting PATH to expose dx(doxius)
ENV PATH="/usr/local/cargo/bin:${PATH}"

WORKDIR /app

COPY . .

# Build frontend
RUN dx build --release --platform web

# Exposing the port for frontend
EXPOSE 8080

CMD ["dx", "serve", "--platform", "web", "--release"]
