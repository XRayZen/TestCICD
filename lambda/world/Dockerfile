FROM rust:1.69.0
WORKDIR /app
RUN curl https://lambda.rustup.rs -sSf | sh -s -- -y && \
    . $HOME/.cargo/env && \
    cargo install cargo-aws-lambda && \
    apt-get update && \
    apt-get install -y zip && \
    rm -rf /var/lib/apt/lists/*
COPY . .
RUN cargo build --release --bin world
RUN cargo test --bin world --no-run
RUN mkdir -p /build && \
    cd target/release && \
    cp world /build/ && \
    cd /build && \
    zip world.zip world
CMD ["bash"]