# Dockerfile for building Rust AWS Lambda function using cargo-aws-lambda

# Set the variable for the lambda function name
ARG LAMBDA_FUNCTION_NAME=hello

# Use the official Rust image as the base image
FROM rust:1.69 as builder

# Set the working directory
WORKDIR /usr/src/${LAMBDA_FUNCTION_NAME}

# Copy the Cargo.toml and Cargo.lock files to the container
COPY Cargo.toml Cargo.lock ./

# Build the Lambda function
RUN cargo install cargo-aws-lambda
RUN cargo lambda build --arm64 --release --bin ${LAMBDA_FUNCTION_NAME}

# Use the official Amazon Linux image as the base image
FROM amazonlinux:2

# Copy the Lambda function binary to the container
COPY --from=builder /usr/src/${LAMBDA_FUNCTION_NAME}/target/lambda/release/${LAMBDA_FUNCTION_NAME} /var/task/

# Set the CMD to the Lambda function handler
CMD [ "${LAMBDA_FUNCTION_NAME}" ]

