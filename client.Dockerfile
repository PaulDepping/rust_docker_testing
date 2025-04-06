FROM rust AS builder
WORKDIR /usr/src/server
COPY . .
RUN rm -rf ./target
RUN cargo build --release --bin client

FROM debian:stable-slim AS main
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*

# Add a new user "john" with user id 8877
RUN useradd basic_user
# Change to non-root privilege
USER basic_user

COPY --from=builder /usr/src/server/target/release/client /usr/local/bin/client


CMD ["client"]
