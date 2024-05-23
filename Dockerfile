# Use a imagem base do Arch Linux
FROM archlinux:latest

# Instale dependências básicas e Rust
RUN pacman -Syu --noconfirm && \
    pacman -S --noconfirm base-devel rustup

# Instale a toolchain do Rust
RUN rustup default stable

# Crie um diretório para o aplicativo
WORKDIR /usr/src/rustinstallarch

# Copie os arquivos Cargo.toml e Cargo.lock
COPY Cargo.toml Cargo.lock ./

# Crie um diretório src e adicione um arquivo main.rs para evitar erros durante o cargo fetch
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Baixe as dependências
RUN cargo fetch

# Remova o diretório src temporário
RUN rm -rf src

# Copie o restante do código
COPY . .

# Compile o projeto
RUN cargo build --release

# Execute o aplicativo
CMD ["./target/release/rustinstallarch"]
