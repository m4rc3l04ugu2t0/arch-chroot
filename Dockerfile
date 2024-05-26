# Use the official Arch Linux base image
FROM archlinux:latest

# Update the package database and install required packages in a single RUN command
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm base-devel curl \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y \
    && vim \
    && pacman -Scc --noconfirm  # Clean up the package cache

# Set the PATH environment variable for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a new directory for our application
WORKDIR /usr/src/rustinstallarch

# Copy the Cargo.toml and Cargo.lock files separately to leverage Docker layer caching
COPY Cargo.toml Cargo.lock ./

# Pre-fetch the dependencies
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Define the entrypoint of the container
CMD ["./target/release/rustinstallarch"]
