# Use the official Arch Linux base image
FROM archlinux:latest

# Update the package database and install required packages
RUN pacman -Syu --noconfirm \
    && pacman -S --noconfirm base-devel

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Create a new directory for our application
WORKDIR /usr/src/rustinstallarch

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Pre-fetch the dependencies
RUN cargo fetch

# Copy the source code
COPY . .

# Build the application
RUN cargo build --release

# Define the entrypoint of the container
CMD ["./target/release/rustinstallarch"]
