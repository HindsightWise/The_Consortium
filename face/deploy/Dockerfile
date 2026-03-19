FROM rust:1.84 as builder

WORKDIR /usr/src/app
COPY . .

# Build the company core
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/the_consortium /usr/local/bin/
COPY --from=builder /usr/src/app/logs /var/log/janus

CMD ["the_consortium"]