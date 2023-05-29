ARG FUNCTION_DIR="/world"
# https://hub.docker.com/layers/library/rust/1.69.0-buster/images/sha256-f86d4e0030192bb12b3464a0bf89fb972e0b653504a867c7d1078fd8677736be?context=explore
# FROM rust:1.69.0-buster as builder
# ARM用ビルドイメージ
# https://hub.docker.com/r/messense/rust-musl-cross/tags?page=1&ordering=last_updated
# https://github.com/rust-cross/rust-musl-cross
FROM messense/rust-musl-cross:aarch64-musl

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

# Cache build folders, see: https://stackoverflow.com/a/60590697/7661119
# Docker Buildkit required
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    # x86用
    # rustup target add x86_64-unknown-linux-musl && \
    # cargo build --release --target x86_64-unknown-linux-musl
    # ARM用 ビルドするために必要なパッケージをインストールしビルド  
    # apt update && \
    # app-get install -y gcc && \
    # apt-get install -y build-essential && \
    # apt-get install -y gcc-multilib && \
    # musl-libcをインストール
    apt-get install -y musl-tools && \
    rustup target add aarch64-unknown-linux-musl && \
    cargo build --release --target aarch64-unknown-linux-musl

# copy artifacts to a clean image
FROM gcr.io/distroless/static

# Set the working directory to function root directory
WORKDIR "/world"

# x86用
# COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/bootstrap bootstrap
# ARM用
COPY --from=builder /build/target/aarch64-unknown-linux-musl/release/bootstrap bootstrap

ENTRYPOINT [ "./bootstrap" ]