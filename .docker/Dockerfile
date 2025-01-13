FROM rust:1.83.0 AS builder
WORKDIR /workspace

# install musl target to link glibc statically
RUN rustup target add x86_64-unknown-linux-musl

# install protoc compiler
RUN apt-get update && \
    apt-get install -y protobuf-compiler && \
    rm -rf /var/lib/apt/lists/*

# Build application
COPY . .
RUN cargo build --target=x86_64-unknown-linux-musl --release


FROM alpine AS runtime
WORKDIR /nebula

# copy executable, data and configuration
COPY --from=builder /workspace/target/x86_64-unknown-linux-musl/release/nebula_registry .

# copy configuration
COPY --from=builder /workspace/nebula_registry/configuration configuration

# copy data - REMARK: Move cifar-10-binary.tar.gz to the data folder and extract it - it will be ignored by git but not by docker
COPY --from=builder /workspace/nebula_registry/data data


EXPOSE 12345
CMD ["/nebula/nebula_registry"]
