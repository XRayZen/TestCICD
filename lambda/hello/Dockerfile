# Dockerfile for building Rust AWS Lambda function using cargo-aws-lambda
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder
# FROM amazonlinux:2 as builder

# Set the working directory
WORKDIR /usr/src/hello

# Copy the Cargo.toml and Cargo.lock files to the container
# COPY Cargo.toml Cargo.lock build.rs ./
# Copy the entire project source to the container
COPY . ./

# Install protoc
RUN apt-get update
RUN apt-get install -y protobuf-compiler

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
RUN rustup update
# RUN rustup component add rustfmt --toolchain nightly

# Build the Lambda function
RUN cargo +nightly lambda build -Z sparse-registry --release --bin hello
RUN ls -l /usr/src/hello

# 第二ステージ
# Use the official Amazon Linux image as the base image
FROM amazonlinux:2

# Copy the Lambda function binary to the container
COPY --from=builder /usr/src/hello/target/lambda/hello /var/task/

# Set the CMD to the Lambda function handler
CMD [ "hello" ]