FROM debian:13-slim

RUN apt update && apt install build-essential curl -y
ENV RUST_VERSION 1.94.1

# Install Rust for building cryptography
RUN     curl --proto '=https' --tlsv1.2 -OO -sSf https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init{,.sha256}
RUN rustupSha256='4acc9acc76d5079515b46346a485974457b5a79893cfb01112423c89aeb5aa10' && echo "${rustupSha256} *rustup-init" | sha256sum -c -;
RUN chmod +x rustup-init 
RUN ./rustup-init --default-toolchain=${RUST_VERSION} -y
RUN echo "source $HOME/.cargo/env" >> $HOME/.bashrc

WORKDIR /io
CMD ["/root/.cargo/bin/cargo", "build", "--release"]
