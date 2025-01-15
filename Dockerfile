# Build stage
FROM rust:1.84 AS builder

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

# Runtime stage
FROM debian:latest

RUN apt-get update && apt-get install -y libpq5 && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/postgre-rust /usr/local/bin/

CMD ["postgre-rust"]
