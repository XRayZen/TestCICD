# Dockerfile for building Rust AWS Lambda function using cargo-aws-lambda
FROM ghcr.io/cargo-lambda/cargo-lambda:latest as builder

# Set the working directory
WORKDIR /usr/src/hello

# Copy the entire project source to the container
COPY . ./

# 依存しているパッケージをインストール
RUN apt-get update
RUN apt-get install -y protobuf-compiler

# ツールチェインをインストール 
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly
RUN rustup update

# Build the Lambda function
RUN cargo +nightly lambda build -Z sparse-registry --release --bin hello

# copy binary to /usr/comp/
RUN mkdir -p /usr/comp/
# COPY /usr/src/hello/target/lambda/hello /usr/comp/
RUN cp -r /usr/src/hello/target/lambda/hello /usr/comp/
# remove target/release
RUN rm -rf /usr/src/hello/target/release
# 出来なかったらワーキングディレクトリすべて削除に変更する

# 第二ステージ
FROM public.ecr.aws/lambda/provided:al2

# Set the working directory
WORKDIR /hello

COPY --from=builder /usr/comp/hello/bootstrap bootstrap

ENTRYPOINT [ "./bootstrap" ]

# Copy the Lambda function binary to the container
# COPY --from=builder /usr/src/hello/target/lambda/hello /var/task/
# COPY --from=builder /usr/comp/hello /var/task/

# # Set the CMD to the Lambda function handler
# CMD [ "hello" ]