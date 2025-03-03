FROM rust:1.85

ARG USERNAME=vscode
ARG USER_UID=1000
ARG USER_GID=$USER_UID

# Create the user
RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME \
    #
    # [Optional] Add sudo support. Omit if you don't need to install software after connecting.
    && apt-get update \
    && apt-get install -y sudo \
    && echo $USERNAME ALL=\(root\) NOPASSWD:ALL > /etc/sudoers.d/$USERNAME \
    && chmod 0440 /etc/sudoers.d/$USERNAME

# ********************************************************
# * Anything else you want to do like clean up goes here *
# ********************************************************

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
RUN chown $USERNAME:$USERNAME /usr/local/cargo/ -R

# [Optional] Set the default user. Omit if you want to keep the default as root.
USER $USERNAME