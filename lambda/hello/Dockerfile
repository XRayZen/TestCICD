ARG FUNCTION_DIR="/hello"
FROM rust:1.69.0-buster as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src
ADD proto proto

# RUN apt-get update
# RUN apt-get install -y protobuf-compiler

# Cache build folders, see: https://stackoverflow.com/a/60590697/7661119
# Docker Buildkit required
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    apt-get update && \ 
    apt-get install -y protobuf-compiler && \
    apt install musl-tools -y && \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# copy artifacts to a clean image
FROM gcr.io/distroless/static

# Include global arg in this stage of the build
ARG FUNCTION_DIR

# Set the working directory to function root directory
WORKDIR ${FUNCTION_DIR}

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/bootstrap bootstrap

ENTRYPOINT [ "./bootstrap" ]