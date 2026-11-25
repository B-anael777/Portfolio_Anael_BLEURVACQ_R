FROM rust:1-slim AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/portfolio_rust /app/portfolio_rust
COPY styles ./styles
COPY js ./js
COPY *.pdf *.docx *.mp4 *.jpg *.jpeg *.png ./
ENV PORT=8080
EXPOSE 8080
CMD ["/app/portfolio_rust"]
