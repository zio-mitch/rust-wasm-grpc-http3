FROM rust:1.94-slim

RUN apt-get update \
    && apt-get install -y protobuf-compiler libprotobuf-dev \
    && rm -rf /var/lib/apt/lists/*