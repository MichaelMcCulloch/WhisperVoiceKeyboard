#!/bin/sh

source .vscode/cargoNdkEnv.sh
cargo ndk --bindgen --target=aarch64-linux-android   --target=x86_64-linux-android   build --release
