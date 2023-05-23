ARG FUNCTION_DIR="/world"
# https://hub.docker.com/layers/library/rust/1.69.0-buster/images/sha256-f86d4e0030192bb12b3464a0bf89fb972e0b653504a867c7d1078fd8677736be?context=explore
FROM rust:1.69.0-buster as builder

WORKDIR /build

ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD src src

# Cache build folders, see: https://stackoverflow.com/a/60590697/7661119
# Docker Buildkit required
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/home/root/app/target \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --target x86_64-unknown-linux-musl

# copy artifacts to a clean image
FROM gcr.io/distroless/static


# Set the working directory to function root directory
ARG FUNCTION_DIR="/world"
WORKDIR ${FUNCTION_DIR}

COPY --from=builder /build/target/x86_64-unknown-linux-musl/release/bootstrap /bootstrap

ENTRYPOINT [ "/bootstrap" ]
# CMD [ "world" ]