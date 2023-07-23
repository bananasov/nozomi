set shell := ["bash", "-uc"]
set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build

default: build

build:
    cargo build --release --target i686-pc-windows-msvc

fmt:
    cargo fmt

clippy:
    cargo clippy