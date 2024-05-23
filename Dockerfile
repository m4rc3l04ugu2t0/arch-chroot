# Use a imagem oficial do Rust como base
FROM rust:latest

# Crie um diretório de trabalho dentro do container
WORKDIR /usr/src/rustinstallarch

# Copie o arquivo Cargo.toml e o arquivo Cargo.lock para o diretório de trabalho
COPY Cargo.toml Cargo.lock ./

# Baixe as dependências
RUN cargo fetch

# Copie o restante do código fonte
COPY . .

# Compile o código
RUN cargo build --release

# Defina o comando padrão para executar o binário resultante
CMD ["./target/release/rustinstallarch"]
