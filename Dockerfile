# ---- build stage ----
    FROM rust:1.86-slim AS builder
    WORKDIR /app
    COPY . .
    RUN cargo build --release
    
    # ---- runtime stage ----
    FROM debian:stable-slim
    WORKDIR /app
    COPY --from=builder /app/target/release/llm-guard .
    EXPOSE 9000
    CMD ["./llm-guard"]
    