#!/bin/bash
cargo build --release --target x86_64-unknown-linux-musl
rm -f lambda_function.zip && zip -j lambda_function.zip ./target/x86_64-unknown-linux-musl/release/bootstrap