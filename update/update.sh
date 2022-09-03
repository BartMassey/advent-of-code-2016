#!/bin/sh
cargo fix --edition --edition-idioms --tests
sed -i '/\[package\]/aedition="2018"' Cargo.toml
cargo fix --edition --edition-idioms --tests --allow-dirty
sed -i 's/2018/2021/' Cargo.toml
cargo upgrade
cargo fmt --all
cargo clippy
