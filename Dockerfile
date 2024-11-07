# Use an official Rust image as a base
FROM rust:latest

# Install mingw-w64 for Windows cross-compilation and other tools
RUN apt-get update && \
    apt-get install -y gcc mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu

# Set a default working directory
WORKDIR /app

# Default command to build the project
CMD ["bash", "-c", "cargo build --release --target=x86_64-pc-windows-gnu"]
