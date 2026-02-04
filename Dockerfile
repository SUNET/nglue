FROM ubuntu:20.04

RUN apt update && apt install build-essential curl -y
ENV RUST_VERSION 1.67.1

# Install Rust for building cryptography
RUN     curl --proto '=https' --tlsv1.2 -OO -sSf https://static.rust-lang.org/rustup/dist/x86_64-unknown-linux-gnu/rustup-init{,.sha256}
RUN rustupSha256='20a06e644b0d9bd2fbdbfd52d42540bdde820ea7df86e92e533c073da0cdd43c' && echo "${rustupSha256} *rustup-init" | sha256sum -c -;
RUN chmod +x rustup-init 
RUN ./rustup-init --default-toolchain=${RUST_VERSION} -y
RUN echo "source $HOME/.cargo/env" >> $HOME/.bashrc

WORKDIR /io
CMD ["/root/.cargo/bin/cargo", "build", "--release"]

