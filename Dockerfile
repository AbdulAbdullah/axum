# First stage: Build the Rust application
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Second stage: Create a minimal image to run the application
FROM debian:bookworm-slim   
WORKDIR /app
COPY --from=builder /app/target/release/axum .
EXPOSE 8000
CMD ["./axum"]
