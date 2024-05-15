#!/bin/bash

# Script de automação para montar partições com Rust

# Verifica se o usuário é root
if [ "$(id -u)" -ne 0 ]; then
    echo "Este script precisa ser executado como root"
    exit 1
fi

# Instala o compilador Rust
pacman -S curl base-devel 
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
exec ../target/debug/rustinstallarch