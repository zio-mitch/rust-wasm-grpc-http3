ARG BASE_IMAGE=rust:1.94-slim
FROM ${BASE_IMAGE}

RUN apt-get update \
    && apt-get install -y binaryen \
    && rm -rf /var/lib/apt/lists/*