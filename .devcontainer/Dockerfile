ARG VARIANT="bullseye"
FROM mcr.microsoft.com/vscode/devcontainers/rust:1-1-${VARIANT}

# get protoc
RUN mkdir /opt/protoc && cd /opt/protoc && \
    wget https://github.com/protocolbuffers/protobuf/releases/download/v25.6/protoc-25.6-linux-x86_64.zip && \
    unzip protoc-25.6-linux-x86_64.zip && \
    rm -rf include protoc-25.6-linux-x86_64.zip readme.txt

# update env
ENV PATH="$PATH:/opt/protoc/bin"

# install clippy, rustfmt and cargo-machete
RUN rustup component add clippy-preview; \
    rustup component add rustfmt; \
    cargo install cargo-machete;

# ensure there are no permission problems in cargo registry
RUN chown vscode:vscode /usr/local/cargo/ -R
