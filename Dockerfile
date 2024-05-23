FROM rust:latest

WORKDIR /usr/src/myapp

COPY Cargo.toml Cargo.lock ./

RUN cargo build 

COPY . .

RUN cargo build --release

CMD [ "./target/release/rustinstallarch" ]