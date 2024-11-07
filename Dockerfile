# Use a slimmer Rust base image
FROM rust:slim

# Install mingw-w64 for Windows cross-compilation and clean up to reduce size
RUN apt-get update && \
    apt-get install -y gcc-mingw-w64 && \
    rustup target add x86_64-pc-windows-gnu && \
    rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the source code into the container
COPY . .

# Build the project for Windows
CMD ["bash", "-c", "cargo build --release --target=x86_64-pc-windows-gnu"]
